local function show_status(message)
  renoise.app():show_status(message)
  print(message)
end

renoise.tool():add_menu_entry {
  name = "Main Menu:Tools:rnsmp",
  invoke = function() hello_world() end 
}

function hello_world()
  local view_builder = renoise.ViewBuilder()
  
  local dialog_title = "rnsmp"
  local dialog_content = view_builder:column {
    view_builder:text { text = "Connect to an rnsmp server." },

    view_builder:textfield {
      width = 120,
      text = "Edit me",
      notifier = function(text)
        show_status(("textfield value changed to '%s'"):
          format(text))
      end
    }
  }

  renoise.app():show_custom_dialog(dialog_title, dialog_content)
end