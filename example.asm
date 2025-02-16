section_bss
    a resb 10
    b resb 5

section_data
    c resb 10 7
    d resb 10 [1,2,3,4,5,6,7,8,9,10]
    e resb 20 "hello, world"

section_text
    pushb 10
