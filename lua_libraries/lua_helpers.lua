-- this value is passed in from Rust
local operating_system = operating_system

-- A simple getter for current operating system.
function get_operating_system()
    return operating_system
end

-- A simple table dump
function dump(o)
    if type(o) == 'table' then
       local s = '{ '
       for k,v in pairs(o) do
          if type(k) ~= 'number' then k = '"'..k..'"' end
          s = s .. '['..k..'] = ' .. dump(v) .. ','
       end
       return s .. '} '
    else
       return tostring(o)
    end
 end