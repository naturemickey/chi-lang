grammar I_chi;

// 因为我是完全手写的lexer和parser，所以无所谓用什么形式的语法来表示chi的方法，所以我用Antlr4的g4文件格式来表示chi，因为：
// 1. 我比较熟悉
// 2. 以后说不定还用得着（比如移植到jvm上）

// 希望有可能实现ll1型文法——因为足够简单——不过现实往往事与愿违——如果要简单，那还不如直接上S表达式了。

// Parser:

functionDeclaration
    : FUNCTION Identifier '(' formalParameterList ')' functionBody
    ;
formalParameterList
    : 
    ;

functionBody
    : block
    ;

block
    : '{' blockStatements? '}'
    ;

blockStatements
	: blockStatement+
	;

blockStatement
	: localObjectDeclarationStatement
	| statement
	;

localObjectDeclarationStatement
	: localObjectDeclaration ';'
	;

localObjectDeclaration
    : LET MUTABLE? Identifier ('=' variableInitializer)?
    ;

variableInitializer
    : IntegerLiteral // TODO 先假设只有整数，后面再改。
    ;

statement
    : ADD // TODO 明天从这里开始
    ;

// Lexer:

// Keywords

INT : 'int' ;
FLOAT : 'float' ;
// 有可能我们并不需要void
// VOID : 'void' ;
// main应该只是一个标识符
// MAIN : 'main' ;
BOOL : 'bool' | 'boolean' ;
// 以下全称做为保留不使用。使用简称。
PUBLIC : 'pub' | 'public' ;
PRIVATE : 'pvt' | 'private';
PROTECTED : 'prtc' | 'protected';
FUNCTION : 'fun' | 'function' ;
LET : 'let' ;
MUTABLE : 'mut' ;
CHARACTER : 'char' ;
OVERRIDE : 'override' ;
TAILREC : 'tailrec' ;
CLASS : 'class' ;
OBJECT : 'object' ;

DATA : 'data' ;
CATCH : 'catch' ;
CONST : 'const' ;
CONTINUE : 'continue' ;
DEFAULT : 'default' ;
DO : 'do' ;
IF : 'if' ;
ELSE : 'else' ;
ENUM : 'enum' ;
EXTENDS : 'extends' ;
FINAL : 'final' ;
FINALLY : 'finally' ;
FOR : 'for';
GOTO : 'goto';
IMPLEMENTS : 'impl' | 'implements';
IMPORT : 'import';
INSTANCEOF : 'instanceof';
INTERFACE : 'interface';
LONG : 'long';
NATIVE : 'native';
NEW : 'new';
PACKAGE : 'pkg' | 'package';
RETURN : 'return';
SHORT : 'short';
STATIC : 'static';
STRICTFP : 'strictfp';
SUPER : 'super';
SWITCH : 'switch';
SYNCHRONIZED : 'synchronized';
THIS : 'this';
THROW : 'throw';
THROWS : 'throws';
TRANSIENT : 'transient';
TRY : 'try';
VOID : 'void';
VOLATILE : 'volatile';
WHILE : 'while';
TRAIT : 'trait';
WITH : 'with' ;
USE : 'use' ;
SEALED : 'sealed';
SELF : 'self' ;
MATCH : 'match' ;
ABSTRACT : 'abstract';
ASSERT : 'assert';
BREAK : 'break';
BYTE : 'byte';
CASE : 'case';
DEFINE : 'def' | 'define' ;
VAL : 'val';
VAR : 'var' ;
TYPE : 'type' ;
LAZY : 'lazy' ;
IMPLICIT : 'implicit';
YIELD : 'yield';
// NULL : 'null';

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

// Boolean Literals

BooleanLiteral
	:	'true'
	|	'false'
	;

// Character Literals

CharacterLiteral
	:	'\'' SingleCharacter '\''
	|	'\'' EscapeSequence '\''
	;

// String Literals

StringLiteral
	:	'"' StringCharacters '"'
	;

fragment
StringCharacters
	:	StringCharacter*
	;

fragment
StringCharacter
	:	~["\\\r\n]
	|	EscapeSequence
	;

// Escape Sequences for Character and String Literals

fragment
EscapeSequence
	:	'\\' [btnfr"'\\]
	|	OctalEscape
    |   UnicodeEscape
	;

fragment
OctalEscape
	:	'\\{' OctalDigit '}'
	|	'\\{' OctalDigit OctalDigit '}'
	|	'\\{' ZeroToThree OctalDigit OctalDigit '}'
	;

fragment
ZeroToThree
	:	[0-3]
	;

fragment
UnicodeEscape
    :   '\\' 'u'+ '{' HexDigit HexDigit HexDigit HexDigit '}'
    ;
fragment
SingleCharacter
	:	~['\\\r\n]
	;

// Integer Literals

// TODO 先只做一个十进制
IntegerLiteral
    : DecimalIntegerLiteral
    ;

fragment
DecimalIntegerLiteral
    : DecimalNumeral [lL]?
    ;

fragment
DecimalNumeral
	: '0'
	| NonZeroDigit Digits?
	;

fragment
OctalDigit
	:	[0-7]
	;

// Floating-Point Literals

FloatingPointLiteral
	:	DecimalFloatingPointLiteral
	;

fragment
DecimalFloatingPointLiteral
	:	Digits '.' Digits? ExponentPart? FloatTypeSuffix?
	|	'.' Digits ExponentPart? FloatTypeSuffix?
	|	Digits ExponentPart FloatTypeSuffix?
	|	Digits FloatTypeSuffix
	;

fragment
ExponentPart
	:	ExponentIndicator SignedInteger
	;

fragment
ExponentIndicator
	:	[eE]
	;

fragment
SignedInteger
	:	Sign? Digits
	;

fragment
Sign
	:	[+-]
	;

fragment
FloatTypeSuffix
	:	[fFdD]
	;

fragment
HexDigit
	:	[0-9a-fA-F]
	;
fragment NonZeroDigit : [1-9]  ;
fragment Digits       : [0-9]+ ;


// 这里标识符暂时只支持这些，还不打算支持更复杂的文字。虽然很多语言都支持“中文”，但似乎各种规范都不希望程序员使用中文。
Identifier               : IdentifierStart IdentifierPart*   ;
fragment IdentifierStart : [_A-Za-z]                         ;
fragment IdentifierPart  : [_A-Za-z0-9]                      ;

WS              : [ \t\r\n\u000C]+  -> skip ;
COMMENT         : '/*' .* '*/'      -> skip ;
LINE_COMMENT    : '//' ~[\r\n]*     -> skip ;
