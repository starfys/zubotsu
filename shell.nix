# Copyright 2019 Steven Sheffey
# This file is part of Zubotsu.
#
# Zubotsu is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# Zubotsu is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with Zubotsu.  If not, see <https:#www.gnu.org/licenses/>.
let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
in
{ pkgs ? import <nixpkgs> { overlays = [ moz_overlay ]; }}:
let
  diesel-cli_custom = pkgs.diesel-cli.override {
    postgresqlSupport = true;
    sqliteSupport = false;
    mysqlSupport = false;
  };
in
pkgs.mkShell {
  RUST_LOG="zubotsu=info";
  buildInputs = with pkgs; [
    # Specific rust branch
    latest.rustChannels.nightly.rust
    # For serenity
    pkgconfig
    openssl
    # For diesel
    postgresql
    diesel-cli_custom
  ];
}

