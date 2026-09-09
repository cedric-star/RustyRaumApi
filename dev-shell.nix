{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    openssl # enthält auch die Entwicklungsdateien
    rustc
    cargo
    sqlx-cli
  ];

}
