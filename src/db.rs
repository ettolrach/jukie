// This file is part of catabile.
//
// cantabile is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// cantabile is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with cantabile. If not,
// see <https://www.gnu.org/licenses/>.

// === DESIGN ===

// == Albums ==
// path: text, primary key,
// name: text,
// artists: text,
// cover: blob,
// year: integer,

// == Track ==
// path: text, primary key,
// album_path: text, foreign key (Albums::path),
// title: text,
// artists: text,
// genre: text,
// year: integer,
// position: integer,
