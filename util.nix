let
  files = builtins.readDir ./.;
  file_list = builtins.attrsToList files;
in
{
  files = file_list;
}
