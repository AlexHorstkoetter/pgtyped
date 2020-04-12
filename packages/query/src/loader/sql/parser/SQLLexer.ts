// Generated from src/loader/sql/grammar/SQLLexer.g4 by ANTLR 4.7.3-SNAPSHOT


import { ATN } from "antlr4ts/atn/ATN";
import { ATNDeserializer } from "antlr4ts/atn/ATNDeserializer";
import { CharStream } from "antlr4ts/CharStream";
import { Lexer } from "antlr4ts/Lexer";
import { LexerATNSimulator } from "antlr4ts/atn/LexerATNSimulator";
import { NotNull } from "antlr4ts/Decorators";
import { Override } from "antlr4ts/Decorators";
import { RuleContext } from "antlr4ts/RuleContext";
import { Vocabulary } from "antlr4ts/Vocabulary";
import { VocabularyImpl } from "antlr4ts/VocabularyImpl";

import * as Utils from "antlr4ts/misc/Utils";


export class SQLLexer extends Lexer {
	public static readonly ID = 1;
	public static readonly OPEN_COMMENT = 2;
	public static readonly WORD = 3;
	public static readonly EOF_STATEMENT = 4;
	public static readonly WSL = 5;
	public static readonly STRING = 6;
	public static readonly PARAM_MARK = 7;
	public static readonly WS = 8;
	public static readonly TRANSFORM_ARROW = 9;
	public static readonly SPREAD = 10;
	public static readonly NAME_TAG = 11;
	public static readonly TYPE_TAG = 12;
	public static readonly OB = 13;
	public static readonly CB = 14;
	public static readonly COMMA = 15;
	public static readonly CLOSE_COMMENT = 16;
	public static readonly COMMENT = 1;

	// tslint:disable:no-trailing-whitespace
	public static readonly channelNames: string[] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN",
	];

	// tslint:disable:no-trailing-whitespace
	public static readonly modeNames: string[] = [
		"DEFAULT_MODE", "COMMENT",
	];

	public static readonly ruleNames: string[] = [
		"QUOT", "ID", "OPEN_COMMENT", "SID", "WORD", "EOF_STATEMENT", "WSL", "STRING", 
		"PARAM_MARK", "CID", "WS", "TRANSFORM_ARROW", "SPREAD", "NAME_TAG", "TYPE_TAG", 
		"OB", "CB", "COMMA", "CLOSE_COMMENT",
	];

	private static readonly _LITERAL_NAMES: Array<string | undefined> = [
		undefined, undefined, "'/*'", undefined, "';'", undefined, undefined, 
		"':'", undefined, "'->'", "'...'", "'@name'", "'@param'", "'('", "')'", 
		"','", "'*/'",
	];
	private static readonly _SYMBOLIC_NAMES: Array<string | undefined> = [
		undefined, "ID", "OPEN_COMMENT", "WORD", "EOF_STATEMENT", "WSL", "STRING", 
		"PARAM_MARK", "WS", "TRANSFORM_ARROW", "SPREAD", "NAME_TAG", "TYPE_TAG", 
		"OB", "CB", "COMMA", "CLOSE_COMMENT",
	];
	public static readonly VOCABULARY: Vocabulary = new VocabularyImpl(SQLLexer._LITERAL_NAMES, SQLLexer._SYMBOLIC_NAMES, []);

	// @Override
	// @NotNull
	public get vocabulary(): Vocabulary {
		return SQLLexer.VOCABULARY;
	}
	// tslint:enable:no-trailing-whitespace


	constructor(input: CharStream) {
		super(input);
		this._interp = new LexerATNSimulator(SQLLexer._ATN, this);
	}

	// @Override
	public get grammarFileName(): string { return "SQLLexer.g4"; }

	// @Override
	public get ruleNames(): string[] { return SQLLexer.ruleNames; }

	// @Override
	public get serializedATN(): string { return SQLLexer._serializedATN; }

	// @Override
	public get channelNames(): string[] { return SQLLexer.channelNames; }

	// @Override
	public get modeNames(): string[] { return SQLLexer.modeNames; }

	public static readonly _serializedATN: string =
		"\x03\uC91D\uCABA\u058D\uAFBA\u4F53\u0607\uEA8B\uC241\x02\x12\x82\b\x01" +
		"\b\x01\x04\x02\t\x02\x04\x03\t\x03\x04\x04\t\x04\x04\x05\t\x05\x04\x06" +
		"\t\x06\x04\x07\t\x07\x04\b\t\b\x04\t\t\t\x04\n\t\n\x04\v\t\v\x04\f\t\f" +
		"\x04\r\t\r\x04\x0E\t\x0E\x04\x0F\t\x0F\x04\x10\t\x10\x04\x11\t\x11\x04" +
		"\x12\t\x12\x04\x13\t\x13\x04\x14\t\x14\x03\x02\x03\x02\x03\x03\x03\x03" +
		"\x07\x03/\n\x03\f\x03\x0E\x032\v\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03" +
		"\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\x06\x06\x06>\n\x06\r\x06\x0E" +
		"\x06?\x03\x07\x03\x07\x03\b\x06\bE\n\b\r\b\x0E\bF\x03\b\x03\b\x03\t\x03" +
		"\t\x03\t\x07\tN\n\t\f\t\x0E\tQ\v\t\x03\t\x03\t\x05\tU\n\t\x03\n\x03\n" +
		"\x03\v\x03\v\x03\v\x03\v\x03\f\x06\f^\n\f\r\f\x0E\f_\x03\f\x03\f\x03\r" +
		"\x03\r\x03\r\x03\x0E\x03\x0E\x03\x0E\x03\x0E\x03\x0F\x03\x0F\x03\x0F\x03" +
		"\x0F\x03\x0F\x03\x0F\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03" +
		"\x10\x03\x11\x03\x11\x03\x12\x03\x12\x03\x13\x03\x13\x03\x14\x03\x14\x03" +
		"\x14\x03\x14\x03\x14\x03O\x02\x02\x15\x04\x02\x02\x06\x02\x03\b\x02\x04" +
		"\n\x02\x02\f\x02\x05\x0E\x02\x06\x10\x02\x07\x12\x02\b\x14\x02\t\x16\x02" +
		"\x02\x18\x02\n\x1A\x02\v\x1C\x02\f\x1E\x02\r \x02\x0E\"\x02\x0F$\x02\x10" +
		"&\x02\x11(\x02\x12\x04\x02\x03\x07\x04\x02C\\c|\x06\x022;C\\aac|\x07\x02" +
		"##%(*;>]_\x80\x05\x02\v\f\x0F\x0F\"\"\x03\x02^^\x02\x84\x02\b\x03\x02" +
		"\x02\x02\x02\n\x03\x02\x02\x02\x02\f\x03\x02\x02\x02\x02\x0E\x03\x02\x02" +
		"\x02\x02\x10\x03\x02\x02\x02\x02\x12\x03\x02\x02\x02\x02\x14\x03\x02\x02" +
		"\x02\x03\x16\x03\x02\x02\x02\x03\x18\x03\x02\x02\x02\x03\x1A\x03\x02\x02" +
		"\x02\x03\x1C\x03\x02\x02\x02\x03\x1E\x03\x02\x02\x02\x03 \x03\x02\x02" +
		"\x02\x03\"\x03\x02\x02\x02\x03$\x03\x02\x02\x02\x03&\x03\x02\x02\x02\x03" +
		"(\x03\x02\x02\x02\x04*\x03\x02\x02\x02\x06,\x03\x02\x02\x02\b3\x03\x02" +
		"\x02\x02\n8\x03\x02\x02\x02\f=\x03\x02\x02\x02\x0EA\x03\x02\x02\x02\x10" +
		"D\x03\x02\x02\x02\x12J\x03\x02\x02\x02\x14V\x03\x02\x02\x02\x16X\x03\x02" +
		"\x02\x02\x18]\x03\x02\x02\x02\x1Ac\x03\x02\x02\x02\x1Cf\x03\x02\x02\x02" +
		"\x1Ej\x03\x02\x02\x02 p\x03\x02\x02\x02\"w\x03\x02\x02\x02$y\x03\x02\x02" +
		"\x02&{\x03\x02\x02\x02(}\x03\x02\x02\x02*+\x07)\x02\x02+\x05\x03\x02\x02" +
		"\x02,0\t\x02\x02\x02-/\t\x03\x02\x02.-\x03\x02\x02\x02/2\x03\x02\x02\x02" +
		"0.\x03\x02\x02\x0201\x03\x02\x02\x021\x07\x03\x02\x02\x0220\x03\x02\x02" +
		"\x0234\x071\x02\x0245\x07,\x02\x0256\x03\x02\x02\x0267\b\x04\x02\x027" +
		"\t\x03\x02\x02\x0289\x05\x06\x03\x029:\x03\x02\x02\x02:;\b\x05\x03\x02" +
		";\v\x03\x02\x02\x02<>\t\x04\x02\x02=<\x03\x02\x02\x02>?\x03\x02\x02\x02" +
		"?=\x03\x02\x02\x02?@\x03\x02\x02\x02@\r\x03\x02\x02\x02AB\x07=\x02\x02" +
		"B\x0F\x03\x02\x02\x02CE\t\x05\x02\x02DC\x03\x02\x02\x02EF\x03\x02\x02" +
		"\x02FD\x03\x02\x02\x02FG\x03\x02\x02\x02GH\x03\x02\x02\x02HI\b\b\x04\x02" +
		"I\x11\x03\x02\x02\x02JT\x05\x04\x02\x02KU\x05\x04\x02\x02LN\v\x02\x02" +
		"\x02ML\x03\x02\x02\x02NQ\x03\x02\x02\x02OP\x03\x02\x02\x02OM\x03\x02\x02" +
		"\x02PR\x03\x02\x02\x02QO\x03\x02\x02\x02RS\n\x06\x02\x02SU\x05\x04\x02" +
		"\x02TK\x03\x02\x02\x02TO\x03\x02\x02\x02U\x13\x03\x02\x02\x02VW\x07<\x02" +
		"\x02W\x15\x03\x02\x02\x02XY\x05\x06\x03\x02YZ\x03\x02\x02\x02Z[\b\v\x03" +
		"\x02[\x17\x03\x02\x02\x02\\^\t\x05\x02\x02]\\\x03\x02\x02\x02^_\x03\x02" +
		"\x02\x02_]\x03\x02\x02\x02_`\x03\x02\x02\x02`a\x03\x02\x02\x02ab\b\f\x04" +
		"\x02b\x19\x03\x02\x02\x02cd\x07/\x02\x02de\x07@\x02\x02e\x1B\x03\x02\x02" +
		"\x02fg\x070\x02\x02gh\x070\x02\x02hi\x070\x02\x02i\x1D\x03\x02\x02\x02" +
		"jk\x07B\x02\x02kl\x07p\x02\x02lm\x07c\x02\x02mn\x07o\x02\x02no\x07g\x02" +
		"\x02o\x1F\x03\x02\x02\x02pq\x07B\x02\x02qr\x07r\x02\x02rs\x07c\x02\x02" +
		"st\x07t\x02\x02tu\x07c\x02\x02uv\x07o\x02\x02v!\x03\x02\x02\x02wx\x07" +
		"*\x02\x02x#\x03\x02\x02\x02yz\x07+\x02\x02z%\x03\x02\x02\x02{|\x07.\x02" +
		"\x02|\'\x03\x02\x02\x02}~\x07,\x02\x02~\x7F\x071\x02\x02\x7F\x80\x03\x02" +
		"\x02\x02\x80\x81\b\x14\x05\x02\x81)\x03\x02\x02\x02\n\x02\x030?FOT_\x06" +
		"\x04\x03\x02\t\x03\x02\b\x02\x02\x04\x02\x02";
	public static __ATN: ATN;
	public static get _ATN(): ATN {
		if (!SQLLexer.__ATN) {
			SQLLexer.__ATN = new ATNDeserializer().deserialize(Utils.toCharArray(SQLLexer._serializedATN));
		}

		return SQLLexer.__ATN;
	}

}

