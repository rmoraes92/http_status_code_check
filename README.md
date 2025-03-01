# HTTP Status Code Checker

dead simple binary to check http status code returned from health-check
endpoint.

## Why not just use cURL?

cURL can definitively do the job as long as you are willing to mix the shell
script to parse the payload. Specially if you want to cover a range of status
instead of a single HTTP code.

The `http_status_code_check` command removes the necessity of parsing the HTTP
status codes and let you dealing only with the program exit code itself:
- 0 - if the status code is inside the list of expected ones
- 1 - if the status code is outside the list of expected ones

## Install

`cargo install http_status_code_check`

## Usage

- Checking for single code:

`http_status_code_check -u http://google.com -s 200`

- Checking for multiple codes:

`http_status_code_check -u http://google.com -s 201,200`


## License

The MIT License (MIT)

Copyright © 2025 Ramon Moraes <ramonmoraes.foss@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
