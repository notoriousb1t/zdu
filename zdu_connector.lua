local luanet = _G.luanet or require("luanet")
luanet.load_assembly("System")
local TcpClient = luanet.import_type("System.Net.Sockets.TcpClient")
local Encoding = luanet.import_type("System.Text.Encoding")
local iso = Encoding.GetEncoding("ISO-8859-1")

local PORT = 42069
local BASE_ADDR = 0x0657
local ADDR_END = 0x067F
local MEM_LEN = ADDR_END - BASE_ADDR + 1

local client_id = nil
local change_number = 0
local last_known_mem = {}

local client = TcpClient()
local connected = false
print("Connecting to ZDU UI at 127.0.0.1:" .. PORT)

-- Keep trying to connect until successful
while not connected do
    local success, err = pcall(function() client:Connect("127.0.0.1", PORT) end)
    if success then
        connected = true
    else
        emu.frameadvance()
    end
end

local stream = client:GetStream()
print("Connected! Waiting for Assign message...")

local function send_msg(msg_str)
    local bytes = iso:GetBytes(msg_str)
    stream:Write(bytes, 0, #msg_str)
    
    local hex = ""
    for i=1, #msg_str do
        hex = hex .. string.format("%02X ", string.byte(msg_str, i))
    end
    print("Sent " .. #msg_str .. " bytes: " .. hex)
end

local function receive_exact(count)
    local received = {}
    local received_len = 0
    while received_len < count do
        if stream.DataAvailable then
            local b = stream:ReadByte()
            if b == -1 then return nil end
            table.insert(received, string.char(b))
            received_len = received_len + 1
        else
            emu.frameadvance()
        end
    end
    return table.concat(received)
end

local function receive_nonblock(count)
    if not stream.DataAvailable then return nil end
    return receive_exact(count)
end

-- Wait for Assign message (10 bytes: 0x00, ServerID(4), Len=4, AssignedID(4))
local assign_data = receive_exact(10)
if assign_data then
    local msg_type = string.byte(assign_data, 1)
    if msg_type == 0x00 then
        client_id = assign_data:sub(7, 10)
        local id_num = (string.byte(client_id, 1) * 16777216) + 
                       (string.byte(client_id, 2) * 65536) + 
                       (string.byte(client_id, 3) * 256) + 
                       string.byte(client_id, 4)
        print("Assigned Client ID: " .. id_num)
    else
        print("Unexpected initial message type: " .. msg_type)
        return
    end
else
    print("Connection closed during handshake.")
    return
end

local function pack_u64_zero()
    return string.char(0, 0, 0, 0, 0, 0, 0, 0)
end

-- Send initial Check message
local check_payload = pack_u64_zero()
local check_msg = string.char(0x01) .. client_id .. string.char(8) .. check_payload
send_msg(check_msg)

-- Initialize local memory tracker
for i = 0, MEM_LEN - 1 do
    last_known_mem[i] = memory.readbyte(BASE_ADDR + i)
end

print("Starting main loop...")

while true do
    -- 1. Read incoming messages
    local header = receive_nonblock(6)
    local written_this_frame = {}
    
    if header then
        local msg_type = string.byte(header, 1)
        local length = string.byte(header, 6)
        
        if length > 0 then
            local payload = receive_exact(length)
            
            if payload and msg_type == 0x02 then
                local offset_count = (length - 8) / 2
                for i = 0, offset_count - 1 do
                    local idx = 9 + (i * 2)
                    local offset = string.byte(payload, idx)
                    local val = string.byte(payload, idx + 1)
                    
                    if offset < MEM_LEN then
                        memory.writebyte(BASE_ADDR + offset, val)
                        last_known_mem[offset] = val
                        written_this_frame[offset] = true
                    end
                end
            end
        end
    end

    -- 2. Transmit local changes
    local updates = {}
    for i = 0, MEM_LEN - 1 do
        if not written_this_frame[i] then
            local current = memory.readbyte(BASE_ADDR + i)
            if current ~= last_known_mem[i] then
                last_known_mem[i] = current
                table.insert(updates, string.char(i) .. string.char(current))
            end
        end
    end
    
    if #updates > 0 then
        local payload_length = 8 + (#updates * 2)
        if payload_length <= 255 then
            local msg = string.char(0x02) .. client_id .. string.char(payload_length) .. pack_u64_zero()
            for _, u in ipairs(updates) do
                msg = msg .. u
            end
            send_msg(msg)
        else
            print("Warning: Too many updates for one message!")
        end
    end

    emu.frameadvance()
end
