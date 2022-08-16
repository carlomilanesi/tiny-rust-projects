@echo off
echo 1.
target\debug\exit_code success
echo %errorlevel%
echo 2.
target\debug\exit_code abort
echo %errorlevel%
echo 3.
target\debug\exit_code panic Ouch!
echo %errorlevel%
echo 4.
target\debug\exit_code exit ok
echo %errorlevel%
echo 5.
target\debug\exit_code exit invalid_argument
echo %errorlevel%
echo 6.
target\debug\exit_code exit too_much_data
echo %errorlevel%
echo 7.
target\debug\exit_code return ok
echo %errorlevel%
echo 8.
target\debug\exit_code return invalid_argument
echo %errorlevel%
echo 9.
target\debug\exit_code return too_much_data
echo %errorlevel%

echo 10.
target\debug\exit_code
echo %errorlevel%
echo 11.
target\debug\exit_code abcd
echo %errorlevel%
echo 12.
target\debug\exit_code panic
echo %errorlevel%
echo 13.
target\debug\exit_code exit
echo %errorlevel%
echo 14.
target\debug\exit_code exit abcd
echo %errorlevel%
echo 15.
target\debug\exit_code return
echo %errorlevel%
echo 16.
target\debug\exit_code return abcd
echo %errorlevel%
