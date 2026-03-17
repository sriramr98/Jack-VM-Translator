//push argument 1
@ARG
D=M
@1
D=D+A
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1


        //pop pointer 1
@SP
M=M-1
A=M
D=M
@THAT
M=D

//push constant 0
@0
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop that 0
                        @SP
                        M=M-1
                        A=M
                        D=M
                        @tmp.1
                        M=D
                        @THAT
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
                        
//push constant 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop that 1
                        @SP
                        M=M-1
                        A=M
                        D=M
                        @tmp.1
                        M=D
                        @THAT
                        D=M
                        @1
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

//push constant 2
@2
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
                        
(LOOP)
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


                // IF-GOTO COMPUTE_ELEMENT
@SP
M=M-1
A=M
D=M
@COMPUTE_ELEMENT
D;JGT


                // GOTO END
@END
0;JMP

(COMPUTE_ELEMENT)
//push that 0
@THAT
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

//push that 1
@THAT
D=M
@1
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
// pop that 2
                        @SP
                        M=M-1
                        A=M
                        D=M
                        @tmp.1
                        M=D
                        @THAT
                        D=M
                        @2
                        D=D+A
                        @tmp2.1
                        M=D
                        @tmp.1
                        D=M
                        @tmp2.1
                        A=M
                        M=D
                        

        //push pointer 1
@THAT
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

        //pop pointer 1
@SP
M=M-1
A=M
D=M
@THAT
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
                        @tmp.2
                        M=D
                        @ARG
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
                        

                // GOTO LOOP
@LOOP
0;JMP

(END)
