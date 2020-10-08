grammar I_chi;

// 因为我是完全手写的lexer和parser，所以无所谓用什么形式的语法来表示chi的方法，所以我用Antlr4的g4文件格式来表示chi，因为：
// 1. 我比较熟悉
// 2. 以后说不定还用得着（比如移植到jvm上）

// 希望有可能实现ll1型文法——因为足够简单——不过现实往往事与愿违——如果要简单，那还不如直接上S表达式了。

// Parser:

functionDeclaration
    : VOID MAIN '(' formalParameterList ')' functionBody
    ;
formalParameterList
    : 
    ;

functionBody
    : block
    ;

block
    : '{' '}'
    ;


// Lexer:

// Keywords

INT : 'int' ;
FLOAT : 'float' ;
VOID : 'void' ;
MAIN : 'main' ;
BOOL : 'bool' ;

// Separators

LPAREN : '(';
RPAREN : ')';
LBRACE : '{';
RBRACE : '}';
LBRACK : '[';
RBRACK : ']';
SEMI : ';';
COMMA : ',';
DOT : '.';

// Operators

ASSIGN : '=';
GT : '>';
LT : '<';
BANG : '!';
TILDE : '~';
QUESTION : '?';
COLON : ':';
EQUAL : '==';
LE : '<=';
GE : '>=';
NOTEQUAL : '!=';
AND : '&&';
OR : '||';
INC : '++';
DEC : '--';
ADD : '+';
SUB : '-';
MUL : '*';
DIV : '/';
BITAND : '&';
BITOR : '|';
CARET : '^';
MOD : '%';
ARROW : '->';
COLONCOLON : '::';

ADD_ASSIGN : '+=';
SUB_ASSIGN : '-=';
MUL_ASSIGN : '*=';
DIV_ASSIGN : '/=';
AND_ASSIGN : '&=';
OR_ASSIGN : '|=';
XOR_ASSIGN : '^=';
MOD_ASSIGN : '%=';
LSHIFT_ASSIGN : '<<=';
RSHIFT_ASSIGN : '>>=';
URSHIFT_ASSIGN : '>>>=';

// 这里标识符暂时只支持这些，还不打算支持更复杂的文字。虽然很多语言都支持“中文”，但似乎各种规范都不希望程序员使用中文。
Identifier      : IdentifierStart IdentifierPart*   ;
IdentifierStart : [_A-Za-z]                         ;
IdentifierPart  : [_A-Za-z0-9]                      ;

WS              : [ \t\r\n\u000C]+  -> skip ;
COMMENT         : '/*' .*? '*/'     -> skip ;
LINE_COMMENT    : '//' ~[\r\n]*     -> skip ;
