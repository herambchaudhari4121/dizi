pub const DEFAULT_KEYMAP: &str = "
[[keymap]]
leys = [ \"q\" ]
command = \"close\"

[[keymap]]
keys = [ \"r\" ]
command = \"reload_dirlist\"

[[keymap]]
keys = [ \"z\", \"h\" ]
command = \"reload_dirlist\"

[[keymap]]
keys = [ \"arrow_up\" ]
command = \"cursor_move_up\"

[[keymap]]
keys = [ \"arrow_down\" ]
command = \"cursor_move_down\"

[[keymap]]
keys = [ \"arrow_left\" ]
command = \"cd ..\"

[[keymap]]
keys = [ \"arrow_right\" ]
command = \"open\"

[[keymap]]
keys = [ \"\n\" ]
command = \"open\"

[[keymap]]
keys = [ \"end\" ]
command = \"cursor_move_end\"

[[keymap]]
keys = [ \"home\" ]
command = \"cursor_move_home\"

[[keymap]]
keys = [ \"page_up\" ]
command = \"cursor_move_page_up\"

[[keymap]]
keys = [ \"page_down\" ]
command = \"cursor_move_page_down\"

[[keymap]]
keys = [ \"\t\" ]
command = \"toggle_view\"

[[keymap]]
keys = [ \"c\", \"d\" ]
command = \":cd \"

[[keymap]]
keys = [ \"t\" ]
command = \"select --all=true --toggle=true\"

[[keymap]]
keys = [ \":\" ]
command = \":\"

[[keymap]]
keys = [ \";\" ]
command = \":\"

[[keymap]]
keys = [ \"?\" ]
command = \":search \"

[[keymap]]
keys = [ \"\\\" ]
command = \":search_glob \"

[[keymap]]
keys = [ \"/\" ]
command = \"search_skim\"

[[keymap]]
keys = [ \"[\" ]
command = \"search_prev\"

[[keymap]]
keys = [ \"]\" ]
command = \"search_next\"

[[keymap]]
keys = [ \"u\", \"r\" ]
command = \"sort reverse\"

[[keymap]]
keys = [ \"u\", \"l\" ]
command = \"sort lexical\"

[[keymap]]
keys = [ \"u\", \"m\" ]
command = \"sort mtime\"

[[keymap]]
keys = [ \"u\", \"n\" ]
command = \"sort natural\"

[[keymap]]
keys = [ \"u\", \"s\" ]
command = \"sort size\"

[[keymap]]
keys = [ \"u\", \"e\" ]
command = \"sort ext\"

[[keymap]]
keys = [ \"g\", \"r\" ]
command = \"cd /\"

[[keymap]]
keys = [ \"g\", \"c\" ]
command = \"cd ~/.config\"

[[keymap]]
keys = [ \"g\", \"d\" ]
command = \"cd ~/Downloads\"

[[keymap]]
keys = [ \"g\", \"e\" ]
command = \"cd /etc\"

[[keymap]]
keys = [ \"g\", \"h\" ]
command = \"cd ~/\"

[[keymap]]
keys = [ \"Q\" ]
command = \"server_request\"
json.request = \"/server/quit\"

[[keymap]]
keys = [ \" \" ]
command = \"server_request\"
json.request = \"/player/toggle/play\"

[[keymap]]
keys = [ \"0\" ]
command = \"server_request\"
json.request = \"/player/volume/increase\"
json.amount = 1

[[keymap]]
keys = [ \"9\" ]
command = \"server_request\"
json.request = \"/player/volume/decrease\"
json.amount = 1

[[keymap]]
keys = [ \"S\" ]
command = \"server_request\"
json.request = \"/player/toggle/shuffle\"

[[keymap]]
keys = [ \"R\" ]
command = \"server_request\"
json.request = \"/player/toggle/repeat\"

[[keymap]]
keys = [ \"N\" ]
command = \"server_request\"
json.request = \"/player/toggle/next\"

[[keymap]]
keys = [ \"n\" ]
command = \"server_request\"
json.request = \"/player/play/next\"

[[keymap]]
keys = [ \"p\" ]
command = \"server_request\"
json.request = \"/player/play/previous\"

[[keymap]]
keys = [ \"a\" ]
command = \"server_request\"
json.request = \"/playlist/append\"

[[keymap]]
keys = [ \"d\" ]
command = \"server_request\"
json.request = \"/playlist/remove\"

[[keymap]]
keys = [ \"w\" ]
command = \"server_request\"
json.request = \"/playlist/move_up\"

[[keymap]]
keys = [ \"s\" ]
command = \"server_request\"
json.request = \"/playlist/move_down\"
";
