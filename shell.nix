# Copyright 2019 Steven Sheffey
# This file is part of Zubotsu.
#
# Foobar is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# Foobar is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with Zubotsu.  If not, see <https:#www.gnu.org/licenses/>.
{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    buildInputs = with pkgs; [openssl pkgconfig];
}
