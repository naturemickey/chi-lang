parser grammar II_chi_Parser;

options {
    tokenVocab=I_chi_Lexer;
}

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