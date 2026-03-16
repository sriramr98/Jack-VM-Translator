//push constant 0
@0
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop local 0
                        @SP
                        M=M-1
                        A=M
                        D=M
                        @tmp.1
                        M=D
                        @LCL
                        D=M
                        @0
                        D=D+A
                        @tmp2.1
                        M=D
                        @tmp.1
                        D=M
                        @tmp2.1
                        A=M
                        M=D
                        
@LOOP
//push argument 0
@ARG
D=M
@0
D=D+A
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1

//push local 0
@LCL
D=M
@0
D=D+A
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1

//add
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D+M
@SP
M=M+1
// pop local 0
                        @SP
                        M=M-1
                        A=M
                        D=M
                        @tmp.2
                        M=D
                        @LCL
                        D=M
                        @0
                        D=D+A
                        @tmp2.2
                        M=D
                        @tmp.2
                        D=M
                        @tmp2.2
                        A=M
                        M=D
                        
//push argument 0
@ARG
D=M
@0
D=D+A
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1

//push constant 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1

//sub
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1
// pop argument 0
                        @SP
                        M=M-1
                        A=M
                        D=M
                        @tmp.1
                        M=D
                        @ARG
                        D=M
                        @0
                        D=D+A
                        @tmp2.1
                        M=D
                        @tmp.1
                        D=M
                        @tmp2.1
                        A=M
                        M=D
                        
//push argument 0
@ARG
D=M
@0
D=D+A
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1


                // load the bottom value from stack
@SP
M=M-1
A=M
D=M
@LOOP
D;JGT

//push local 0
@LCL
D=M
@0
D=D+A
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1

