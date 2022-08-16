echo -n "1. "; target/debug/exit_code success; echo $?
echo -n "2. "; target/debug/exit_code abort; echo $?
echo -n "3. "; target/debug/exit_code panic Ouch!; echo $?
echo -n "4. "; target/debug/exit_code exit ok; echo $?
echo -n "5. "; target/debug/exit_code exit invalid_argument; echo $?
echo -n "6. "; target/debug/exit_code exit too_much_data; echo $?
echo -n "7. "; target/debug/exit_code return ok; echo $?
echo -n "8. "; target/debug/exit_code return invalid_argument; echo $?
echo -n "9. "; target/debug/exit_code return too_much_data; echo $?

echo -n "10. "; target/debug/exit_code; echo $?
echo -n "11. "; target/debug/exit_code abcd; echo $?
echo -n "12. "; target/debug/exit_code panic; echo $?
echo -n "13. "; target/debug/exit_code exit; echo $?
echo -n "14. "; target/debug/exit_code exit abcd; echo $?
echo -n "15. "; target/debug/exit_code return; echo $?
echo -n "16. "; target/debug/exit_code return abcd; echo $?

