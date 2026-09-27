package token

// Used to emulate enums, such a useless languaze
type TokenType string

type Token struct {
	Type    TokenType
	Literal string
}

const (
	// Sentinel Tokens
	ILLEGAL TokenType = "ILLEGAL"
	EOF               = "EOF"

	// Meta Tokens
	IDENT = "IDENT"
	INT   = "INT"

	// Operators
	ASSIGN    = "="
	PLUS      = "+"
	MINUS     = "-"
	BANG      = "!"
	ASTERISK  = "*"
	SLASH     = "/"
	LT        = "<"
	GT        = ">"
	COMMA     = ","
	SEMICOLON = ";"
	LPAREN    = "("
	RPAREN    = ")"
	LBRACE    = "{"
	RBRACE    = "}"

	// Long Keywords
	FUNCTION = "FUNCTION"
	LET      = "LET"
	TRUE     = "TRUE"
	FALSE    = "FALSE"
	IF       = "IF"
	ELSE     = "ELSE"
	RETURN   = "RETURN"
	EQ       = "EQ"
	NOT_EQ   = "NOT_EQ"
)

// No way of defining const global hashmaps because enums
// by their nature are mutable in go
var TokenMap = map[string]TokenType{
	"=": ASSIGN,
	"==": EQ,
	"!=": NOT_EQ,
	";": SEMICOLON,
	"(": LPAREN,
	")": RPAREN,
	",": COMMA,
	"+": PLUS,
	"{": LBRACE,
	"}": RBRACE,
	"-": MINUS,
	"!": BANG,
	"*": ASTERISK,
	"/": SLASH,
	"<": LT,
	">": GT,
}

var keywords = map[string]TokenType{
	"fn":     FUNCTION,
	"let":    LET,
	"true":   TRUE,
	"false":  FALSE,
	"if":     IF,
	"else":   ELSE,
	"return": RETURN,
}

func LookupIdent(ident string) TokenType {
	if tok, ok := keywords[ident]; ok {
		return tok
	}
	return IDENT
}

func CreateToken(t TokenType, literal string) Token {
	return Token{
		t,
		literal,
	}
}

func NewToken(tokenType TokenType, ch byte) Token {
	return Token{Type: tokenType, Literal: string(ch)}
}
