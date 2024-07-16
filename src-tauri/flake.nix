{
	inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
	outputs = { nixpkgs, ... }:
	let pkgs = nixpkgs.legacyPackages.x86_64-linux; in {
		packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
			pname = "cwe-client";
			version = "0.1.0";

			nativeBuildInputs = [ pkgs.pkg-config ];

			buildInputs = with pkgs; [
				curl
				wget
				dbus
				openssl_3
				glib
				gtk3
				libsoup
				webkitgtk
				librsvg
			];

			src = ./.;

			cargoHash = "sha256-Tpz4ZW2JjF0qb+J2ejhZfas1TzdZ8oqJwqiuDXOOo7k=";
		};
	};
}
