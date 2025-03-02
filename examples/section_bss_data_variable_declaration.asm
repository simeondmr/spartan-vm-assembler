section_bss
    a resb 2
    b resb 3
    c resw 2
    d resd 1
section_data
    e resb 4 6
    f resb 3 [1, 2, 3]
    g resb 13 "hello, world"
    h resb 13 ['h', 'e', 'l', 'l', 'o',',' ,' ', 'w', 'o', 'r', 'l', 'd', '\0']//edef