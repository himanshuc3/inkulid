package lexer

import (
	"github.com/himanshuc3/inkulid/src/internal/token"
)

type Lexer struct {
	input        string
	position     int
	readPosition int
	ch           byte
}

// Personally, I would keep constructors with prefix New
// as a way of helping developers new to go semantics
func New(input string) *Lexer {
	l := &Lexer{input: input}
	l.readChar()
	return l
}

// Posi is 1 step behind readPosi
func (l *Lexer) readChar() {
	if l.readPosition >= len(l.input) {
		// ASCII code for NUL character
		l.ch = 0
	} else {
		// Since the language only accepts ascii,
		// we won't need to handle runes
		l.ch = l.input[l.readPosition]
	}
	l.position = l.readPosition
	l.readPosition += 1
}

// Currently taking the greedy approach of selecting one
// character at a time instead of multi-char tokens
// Handling linear flow with less nested branching is better than
// more linear branches???
func (l *Lexer) NextToken() token.Token {
	var tok token.Token
	l.skipWhitespace()
	if extractedToken, ok := token.TokenMap[string(l.ch)]; ok {
		tok = token.NewToken(extractedToken, l.ch)
		tok = l.handleAmbiguousTokens(tok)
	} else {
		switch l.ch {
		case 0:
			tok.Literal = ""
			tok.Type = token.EOF
		default:
			// Probably not the current structuring of the code given
			// we have an inconsistency in character position between reading
			// different tokens
			if isLetter(l.ch) {
				tok.Literal = l.readIdentifier()
				tok.Type = token.LookupIdent(tok.Literal)
				return tok
			} else if isDigit(l.ch) {
				tok.Type = token.INT
				tok.Literal = l.readNumber()
				return tok
			} else {
				tok = token.NewToken(token.ILLEGAL, l.ch)
			}
		}
	}

	l.readChar()
	return tok

}

func (l *Lexer) handleAmbiguousTokens(tok token.Token) token.Token {
	literal := tok.Literal + string(l.peekChar())
	if tokenType, ok := token.TokenMap[literal]; ok {
		l.readChar()
		// NOTE: Should use NewToken constuctor instead of Raw
		return token.Token{Type: tokenType, Literal: literal}
	}
	return tok

}

func (l *Lexer) readNumber() string {
	// Position is really helpful in slicing without
	// the need to introduce sentinels
	position := l.position
	for isDigit(l.ch) {
		l.readChar()
	}
	return l.input[position:l.position]
}

func (l *Lexer) skipWhitespace() {
	for l.ch == ' ' || l.ch == '\n' || l.ch == '\t' || l.ch == '\r' {
		l.readChar()
	}
}

func (l *Lexer) readIdentifier() string {
	position := l.position
	for isLetter(l.ch) {
		l.readChar()
	}
	return l.input[position:l.position]
}

// Indexing a string, returns a byte because there is no char
// As an alternative, we can use runes
func (l *Lexer) peekChar() byte {
	if l.readPosition >= len(l.input) {
		return 0
	} else {
		return l.input[l.readPosition]
	}
}

func isLetter(ch byte) bool {
	return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

func isDigit(ch byte) bool {
	return '0' <= ch && ch <= '9'
}
