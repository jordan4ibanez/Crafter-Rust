-- This value is passed in from Rust.
local 
operating_system, current_working_directory
= 
operating_system, current_working_directory

-- A simple getter for current operating system.
function get_operating_system()
    return operating_system
end

-- A simple getter for current working directory.
function get_working_directory()
   return current_working_directory
end

-- A simple table dump.
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

-- Pre-define air.
function register_air()
   crafter.register_block({
       name = "air",
       draw_type = "airlike"
   })
end