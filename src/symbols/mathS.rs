use crate::symbols::public::*;

pub fn define_all_math_symbols() -> HashMap<String, Symbol> {
    use Font::*; //
    use Group::*;
    let mut math = HashMap::new();
    defineSymbolM!(math, main, rel, "\u{2261}", "\\equiv", true);
    defineSymbolM!(math, main, rel, "\u{227a}", "\\prec", true);
    defineSymbolM!(math, main, rel, "\u{227b}", "\\succ", true);
    defineSymbolM!(math, main, rel, "\u{223c}", "\\sim", true);
    defineSymbolM!(math, main, rel, "\u{22a5}", "\\perp", false);
    defineSymbolM!(math, main, rel, "\u{2aaf}", "\\preceq", true);
    defineSymbolM!(math, main, rel, "\u{2ab0}", "\\succeq", true);
    defineSymbolM!(math, main, rel, "\u{2243}", "\\simeq", true);
    defineSymbolM!(math, main, rel, "\u{2223}", "\\mid", true);
    defineSymbolM!(math, main, rel, "\u{226a}", "\\ll", true);
    defineSymbolM!(math, main, rel, "\u{226b}", "\\gg", true);
    defineSymbolM!(math, main, rel, "\u{224d}", "\\asymp", true);
    defineSymbolM!(math, main, rel, "\u{2225}", "\\parallel", false);
    defineSymbolM!(math, main, rel, "\u{22c8}", "\\bowtie", true);
    defineSymbolM!(math, main, rel, "\u{2323}", "\\smile", true);
    defineSymbolM!(math, main, rel, "\u{2291}", "\\sqsubseteq", true);
    defineSymbolM!(math, main, rel, "\u{2292}", "\\sqsupseteq", true);
    defineSymbolM!(math, main, rel, "\u{2250}", "\\doteq", true);
    defineSymbolM!(math, main, rel, "\u{2322}", "\\frown", true);
    defineSymbolM!(math, main, rel, "\u{220b}", "\\ni", true);
    defineSymbolM!(math, main, rel, "\u{221d}", "\\propto", true);
    defineSymbolM!(math, main, rel, "\u{22a2}", "\\vdash", true);
    defineSymbolM!(math, main, rel, "\u{22a3}", "\\dashv", true);
    defineSymbolM!(math, main, rel, "\u{220b}", "\\owns", false);
    defineSymbolM!(math, main, punct, "\u{002e}", "\\ldotp", false);
    defineSymbolM!(math, main, punct, "\u{22c5}", "\\cdotp", false);
    defineSymbolM!(math, main, textord, "\u{0023}", "\\#", false);
    defineSymbolM!(math, main, textord, "\u{0026}", "\\&", false);
    defineSymbolM!(math, main, textord, "\u{2135}", "\\aleph", true);
    defineSymbolM!(math, main, textord, "\u{2200}", "\\forall", true);
    defineSymbolM!(math, main, textord, "\u{210f}", "\\hbar", true);
    defineSymbolM!(math, main, textord, "\u{2203}", "\\exists", true);
    defineSymbolM!(math, main, textord, "\u{2207}", "\\nabla", true);
    defineSymbolM!(math, main, textord, "\u{266d}", "\\flat", true);
    defineSymbolM!(math, main, textord, "\u{2113}", "\\ell", true);
    defineSymbolM!(math, main, textord, "\u{266e}", "\\natural", true);
    defineSymbolM!(math, main, textord, "\u{2663}", "\\clubsuit", true);
    defineSymbolM!(math, main, textord, "\u{2118}", "\\wp", true);
    defineSymbolM!(math, main, textord, "\u{266f}", "\\sharp", true);
    defineSymbolM!(math, main, textord, "\u{2662}", "\\diamondsuit", true);
    defineSymbolM!(math, main, textord, "\u{211c}", "\\Re", true);
    defineSymbolM!(math, main, textord, "\u{2661}", "\\heartsuit", true);
    defineSymbolM!(math, main, textord, "\u{2111}", "\\Im", true);
    defineSymbolM!(math, main, textord, "\u{2660}", "\\spadesuit", true);
    defineSymbolM!(math, main, textord, "\u{00a7}", "\\S", true);
    defineSymbolM!(math, main, textord, "\u{00b6}", "\\P", true);
    defineSymbolM!(math, main, textord, "\u{2020}", "\\dag", false);
    defineSymbolM!(math, main, textord, "\u{2021}", "\\ddag", false);
    defineSymbolM!(math, main, close, "\u{23b1}", "\\rmoustache", true);
    defineSymbolM!(math, main, open, "\u{23b0}", "\\lmoustache", true);
    defineSymbolM!(math, main, close, "\u{27ef}", "\\rgroup", true);
    defineSymbolM!(math, main, open, "\u{27ee}", "\\lgroup", true);
    defineSymbolM!(math, main, bin, "\u{2213}", "\\mp", true);
    defineSymbolM!(math, main, bin, "\u{2296}", "\\ominus", true);
    defineSymbolM!(math, main, bin, "\u{228e}", "\\uplus", true);
    defineSymbolM!(math, main, bin, "\u{2293}", "\\sqcap", true);
    defineSymbolM!(math, main, bin, "\u{2217}", "\\ast", false);
    defineSymbolM!(math, main, bin, "\u{2294}", "\\sqcup", true);
    defineSymbolM!(math, main, bin, "\u{25ef}", "\\bigcirc", true);
    defineSymbolM!(math, main, bin, "\u{2219}", "\\bullet", true);
    defineSymbolM!(math, main, bin, "\u{2021}", "\\ddagger", false);
    defineSymbolM!(math, main, bin, "\u{2240}", "\\wr", true);
    defineSymbolM!(math, main, bin, "\u{2a3f}", "\\amalg", false);
    defineSymbolM!(math, main, bin, "\u{0026}", "\\And", false);
    defineSymbolM!(math, main, rel, "\u{27f5}", "\\longleftarrow", true);
    defineSymbolM!(math, main, rel, "\u{21d0}", "\\Leftarrow", true);
    defineSymbolM!(math, main, rel, "\u{27f8}", "\\Longleftarrow", true);
    defineSymbolM!(math, main, rel, "\u{27f6}", "\\longrightarrow", true);
    defineSymbolM!(math, main, rel, "\u{21d2}", "\\Rightarrow", true);
    defineSymbolM!(math, main, rel, "\u{27f9}", "\\Longrightarrow", true);
    defineSymbolM!(math, main, rel, "\u{2194}", "\\leftrightarrow", true);
    defineSymbolM!(math, main, rel, "\u{27f7}", "\\longleftrightarrow", true);
    defineSymbolM!(math, main, rel, "\u{21d4}", "\\Leftrightarrow", true);
    defineSymbolM!(math, main, rel, "\u{27fa}", "\\Longleftrightarrow", true);
    defineSymbolM!(math, main, rel, "\u{21a6}", "\\mapsto", true);
    defineSymbolM!(math, main, rel, "\u{27fc}", "\\longmapsto", true);
    defineSymbolM!(math, main, rel, "\u{2197}", "\\nearrow", true);
    defineSymbolM!(math, main, rel, "\u{21a9}", "\\hookleftarrow", true);
    defineSymbolM!(math, main, rel, "\u{21aa}", "\\hookrightarrow", true);
    defineSymbolM!(math, main, rel, "\u{2198}", "\\searrow", true);
    defineSymbolM!(math, main, rel, "\u{21bc}", "\\leftharpoonup", true);
    defineSymbolM!(math, main, rel, "\u{21c0}", "\\rightharpoonup", true);
    defineSymbolM!(math, main, rel, "\u{2199}", "\\swarrow", true);
    defineSymbolM!(math, main, rel, "\u{21bd}", "\\leftharpoondown", true);
    defineSymbolM!(math, main, rel, "\u{21c1}", "\\rightharpoondown", true);
    defineSymbolM!(math, main, rel, "\u{2196}", "\\nwarrow", true);
    defineSymbolM!(math, main, rel, "\u{21cc}", "\\rightleftharpoons", true);
    defineSymbolM!(math, ams, rel, "\u{226e}", "\\nless", true);
    defineSymbolM!(math, ams, rel, "\u{e010}", "\\@nleqslant");
    defineSymbolM!(math, ams, rel, "\u{e011}", "\\@nleqq", false);
    defineSymbolM!(math, ams, rel, "\u{2a87}", "\\lneq", true);
    defineSymbolM!(math, ams, rel, "\u{2268}", "\\lneqq", true);
    defineSymbolM!(math, ams, rel, "\u{e00c}", "\\@lvertneqq", false);
    defineSymbolM!(math, ams, rel, "\u{22e6}", "\\lnsim", true);
    defineSymbolM!(math, ams, rel, "\u{2a89}", "\\lnapprox", true);
    defineSymbolM!(math, ams, rel, "\u{2280}", "\\nprec", true);
    defineSymbolM!(math, ams, rel, "\u{22e0}", "\\npreceq", true);
    defineSymbolM!(math, ams, rel, "\u{22e8}", "\\precnsim", true);
    defineSymbolM!(math, ams, rel, "\u{2ab9}", "\\precnapprox", true);
    defineSymbolM!(math, ams, rel, "\u{2241}", "\\nsim", true);
    defineSymbolM!(math, ams, rel, "\u{e006}", "\\@nshortmid", false);
    defineSymbolM!(math, ams, rel, "\u{2224}", "\\nmid", true);
    defineSymbolM!(math, ams, rel, "\u{22ac}", "\\nvdash", true);
    defineSymbolM!(math, ams, rel, "\u{22ad}", "\\nvDash", true);
    defineSymbolM!(math, ams, rel, "\u{22ea}", "\\ntriangleleft", false);
    defineSymbolM!(math, ams, rel, "\u{22ec}", "\\ntrianglelefteq", true);
    defineSymbolM!(math, ams, rel, "\u{228a}", "\\subsetneq", true);
    defineSymbolM!(math, ams, rel, "\u{e01a}", "\\@varsubsetneq", false);
    defineSymbolM!(math, ams, rel, "\u{2acb}", "\\subsetneqq", true);
    defineSymbolM!(math, ams, rel, "\u{e017}", "\\@varsubsetneqq", false);
    defineSymbolM!(math, ams, rel, "\u{226f}", "\\ngtr", true);
    defineSymbolM!(math, ams, rel, "\u{e00f}", "\\@ngeqslant", false);
    defineSymbolM!(math, ams, rel, "\u{e00e}", "\\@ngeqq", false);
    defineSymbolM!(math, ams, rel, "\u{2a88}", "\\gneq", true);
    defineSymbolM!(math, ams, rel, "\u{2269}", "\\gneqq", true);
    defineSymbolM!(math, ams, rel, "\u{e00d}", "\\@gvertneqq", false);
    defineSymbolM!(math, ams, rel, "\u{22e7}", "\\gnsim", true);
    defineSymbolM!(math, ams, rel, "\u{2a8a}", "\\gnapprox", true);
    defineSymbolM!(math, ams, rel, "\u{2281}", "\\nsucc", true);
    defineSymbolM!(math, ams, rel, "\u{22e1}", "\\nsucceq", true);
    defineSymbolM!(math, ams, rel, "\u{22e9}", "\\succnsim", true);
    defineSymbolM!(math, ams, rel, "\u{2aba}", "\\succnapprox", true);
    defineSymbolM!(math, ams, rel, "\u{2246}", "\\ncong", true);
    defineSymbolM!(math, ams, rel, "\u{e007}", "\\@nshortparallel", false);
    defineSymbolM!(math, ams, rel, "\u{2226}", "\\nparallel", true);
    defineSymbolM!(math, ams, rel, "\u{22af}", "\\nVDash", true);
    defineSymbolM!(math, ams, rel, "\u{22eb}", "\\ntriangleright", false);
    defineSymbolM!(math, ams, rel, "\u{22ed}", "\\ntrianglerighteq", true);
    defineSymbolM!(math, ams, rel, "\u{e018}", "\\@nsupseteqq", false);
    defineSymbolM!(math, ams, rel, "\u{228b}", "\\supsetneq", true);
    defineSymbolM!(math, ams, rel, "\u{e01b}", "\\@varsupsetneq", false);
    defineSymbolM!(math, ams, rel, "\u{2acc}", "\\supsetneqq", true);
    defineSymbolM!(math, ams, rel, "\u{e019}", "\\@varsupsetneqq", false);
    defineSymbolM!(math, ams, rel, "\u{22ae}", "\\nVdash", true);
    defineSymbolM!(math, ams, rel, "\u{2ab5}", "\\precneqq", true);
    defineSymbolM!(math, ams, rel, "\u{2ab6}", "\\succneqq", true);
    defineSymbolM!(math, ams, rel, "\u{e016}", "\\@nsubseteqq", false);
    defineSymbolM!(math, ams, bin, "\u{22b4}", "\\unlhd", false);
    defineSymbolM!(math, ams, bin, "\u{22b5}", "\\unrhd", false);
    defineSymbolM!(math, ams, rel, "\u{219a}", "\\nleftarrow", true);
    defineSymbolM!(math, ams, rel, "\u{219b}", "\\nrightarrow", true);
    defineSymbolM!(math, ams, rel, "\u{21cd}", "\\nLeftarrow", true);
    defineSymbolM!(math, ams, rel, "\u{21cf}", "\\nRightarrow", true);
    defineSymbolM!(math, ams, rel, "\u{21ae}", "\\nleftrightarrow", true);
    defineSymbolM!(math, ams, rel, "\u{21ce}", "\\nLeftrightarrow", true);
    defineSymbolM!(math, ams, rel, "\u{25b3}", "\\vartriangle", false);
    defineSymbolM!(math, ams, textord, "\u{210f}", "\\hslash", false);
    defineSymbolM!(math, ams, textord, "\u{25bd}", "\\triangledown", false);
    defineSymbolM!(math, ams, textord, "\u{25ca}", "\\lozenge", false);
    defineSymbolM!(math, ams, textord, "\u{24c8}", "\\circledS", false);
    defineSymbolM!(math, ams, textord, "\u{00ae}", "\\circledR", false);
    defineSymbolM!(math, ams, textord, "\u{2221}", "\\measuredangle", true);
    defineSymbolM!(math, ams, textord, "\u{2204}", "\\nexists", false);
    defineSymbolM!(math, ams, textord, "\u{2127}", "\\mho", false);
    defineSymbolM!(math, ams, textord, "\u{2132}", "\\Finv", true);
    defineSymbolM!(math, ams, textord, "\u{2141}", "\\Game", true);
    defineSymbolM!(math, ams, textord, "\u{2035}", "\\backprime", false);
    defineSymbolM!(math, ams, textord, "\u{25b2}", "\\blacktriangle", false);
    defineSymbolM!(math, ams, textord, "\u{25bc}", "\\blacktriangledown", false);
    defineSymbolM!(math, ams, textord, "\u{25a0}", "\\blacksquare", false);
    defineSymbolM!(math, ams, textord, "\u{29eb}", "\\blacklozenge", false);
    defineSymbolM!(math, ams, textord, "\u{2605}", "\\bigstar", false);
    defineSymbolM!(math, ams, textord, "\u{2222}", "\\sphericalangle", true);
    defineSymbolM!(math, ams, textord, "\u{2201}", "\\complement", true);
    defineSymbolM!(math, ams, textord, "\u{00f0}", "\\eth", true);
    defineSymbolM!(math, ams, textord, "\u{2571}", "\\diagup", false);
    defineSymbolM!(math, ams, textord, "\u{2572}", "\\diagdown", false);
    defineSymbolM!(math, ams, textord, "\u{25a1}", "\\square", false);
    defineSymbolM!(math, ams, textord, "\u{25a1}", "\\Box", false);
    defineSymbolM!(math, ams, textord, "\u{25ca}", "\\Diamond", false);
    defineSymbolM!(math, ams, textord, "\u{00a5}", "\\yen", true);
    defineSymbolM!(math, ams, textord, "\u{2713}", "\\checkmark", true);
    defineSymbolM!(math, ams, textord, "\u{2136}", "\\beth", true);
    defineSymbolM!(math, ams, textord, "\u{2138}", "\\daleth", true);
    defineSymbolM!(math, ams, textord, "\u{2137}", "\\gimel", true);
    defineSymbolM!(math, ams, textord, "\u{03dd}", "\\digamma", true);
    defineSymbolM!(math, ams, textord, "\u{03f0}", "\\varkappa", false);
    defineSymbolM!(math, ams, open, "\u{250c}", "\\@ulcorner", true);
    defineSymbolM!(math, ams, close, "\u{2510}", "\\@urcorner", true);
    defineSymbolM!(math, ams, open, "\u{2514}", "\\@llcorner", true);
    defineSymbolM!(math, ams, close, "\u{2518}", "\\@lrcorner", true);
    defineSymbolM!(math, ams, rel, "\u{2266}", "\\leqq", true);
    defineSymbolM!(math, ams, rel, "\u{2a7d}", "\\leqslant", true);
    defineSymbolM!(math, ams, rel, "\u{2a95}", "\\eqslantless", true);
    defineSymbolM!(math, ams, rel, "\u{2272}", "\\lesssim", true);
    defineSymbolM!(math, ams, rel, "\u{2a85}", "\\lessapprox", true);
    defineSymbolM!(math, ams, rel, "\u{224a}", "\\approxeq", true);
    defineSymbolM!(math, ams, bin, "\u{22d6}", "\\lessdot", false);
    defineSymbolM!(math, ams, rel, "\u{22d8}", "\\lll", true);
    defineSymbolM!(math, ams, rel, "\u{2276}", "\\lessgtr", true);
    defineSymbolM!(math, ams, rel, "\u{22da}", "\\lesseqgtr", true);
    defineSymbolM!(math, ams, rel, "\u{2a8b}", "\\lesseqqgtr", true);
    defineSymbolM!(math, ams, rel, "\u{2251}", "\\doteqdot", false);
    defineSymbolM!(math, ams, rel, "\u{2253}", "\\risingdotseq", true);
    defineSymbolM!(math, ams, rel, "\u{2252}", "\\fallingdotseq", true);
    defineSymbolM!(math, ams, rel, "\u{223d}", "\\backsim", true);
    defineSymbolM!(math, ams, rel, "\u{22cd}", "\\backsimeq", true);
    defineSymbolM!(math, ams, rel, "\u{2ac5}", "\\subseteqq", true);
    defineSymbolM!(math, ams, rel, "\u{22d0}", "\\Subset", true);
    defineSymbolM!(math, ams, rel, "\u{228f}", "\\sqsubset", true);
    defineSymbolM!(math, ams, rel, "\u{227c}", "\\preccurlyeq", true);
    defineSymbolM!(math, ams, rel, "\u{22de}", "\\curlyeqprec", true);
    defineSymbolM!(math, ams, rel, "\u{227e}", "\\precsim", true);
    defineSymbolM!(math, ams, rel, "\u{2ab7}", "\\precapprox", true);
    defineSymbolM!(math, ams, rel, "\u{22b2}", "\\vartriangleleft", false);
    defineSymbolM!(math, ams, rel, "\u{22b4}", "\\trianglelefteq", false);
    defineSymbolM!(math, ams, rel, "\u{22a8}", "\\vDash", true);
    defineSymbolM!(math, ams, rel, "\u{22aa}", "\\Vvdash", true);
    defineSymbolM!(math, ams, rel, "\u{2323}", "\\smallsmile", false);
    defineSymbolM!(math, ams, rel, "\u{2322}", "\\smallfrown", false);
    defineSymbolM!(math, ams, rel, "\u{224f}", "\\bumpeq", true);
    defineSymbolM!(math, ams, rel, "\u{224e}", "\\Bumpeq", true);
    defineSymbolM!(math, ams, rel, "\u{2267}", "\\geqq", true);
    defineSymbolM!(math, ams, rel, "\u{2a7e}", "\\geqslant", true);
    defineSymbolM!(math, ams, rel, "\u{2a96}", "\\eqslantgtr", true);
    defineSymbolM!(math, ams, rel, "\u{2273}", "\\gtrsim", true);
    defineSymbolM!(math, ams, rel, "\u{2a86}", "\\gtrapprox", true);
    defineSymbolM!(math, ams, bin, "\u{22d7}", "\\gtrdot", false);
    defineSymbolM!(math, ams, rel, "\u{22d9}", "\\ggg", true);
    defineSymbolM!(math, ams, rel, "\u{2277}", "\\gtrless", true);
    defineSymbolM!(math, ams, rel, "\u{22db}", "\\gtreqless", true);
    defineSymbolM!(math, ams, rel, "\u{2a8c}", "\\gtreqqless", true);
    defineSymbolM!(math, ams, rel, "\u{2256}", "\\eqcirc", true);
    defineSymbolM!(math, ams, rel, "\u{2257}", "\\circeq", true);
    defineSymbolM!(math, ams, rel, "\u{225c}", "\\triangleq", true);
    defineSymbolM!(math, ams, rel, "\u{223c}", "\\thicksim", false);
    defineSymbolM!(math, ams, rel, "\u{2248}", "\\thickapprox", false);
    defineSymbolM!(math, ams, rel, "\u{2ac6}", "\\supseteqq", true);
    defineSymbolM!(math, ams, rel, "\u{22d1}", "\\Supset", true);
    defineSymbolM!(math, ams, rel, "\u{2290}", "\\sqsupset", true);
    defineSymbolM!(math, ams, rel, "\u{227d}", "\\succcurlyeq", true);
    defineSymbolM!(math, ams, rel, "\u{22df}", "\\curlyeqsucc", true);
    defineSymbolM!(math, ams, rel, "\u{227f}", "\\succsim", true);
    defineSymbolM!(math, ams, rel, "\u{2ab8}", "\\succapprox", true);
    defineSymbolM!(math, ams, rel, "\u{22b3}", "\\vartriangleright", false);
    defineSymbolM!(math, ams, rel, "\u{22b5}", "\\trianglerighteq", false);
    defineSymbolM!(math, ams, rel, "\u{22a9}", "\\Vdash", true);
    defineSymbolM!(math, ams, rel, "\u{2223}", "\\shortmid", false);
    defineSymbolM!(math, ams, rel, "\u{2225}", "\\shortparallel", false);
    defineSymbolM!(math, ams, rel, "\u{226c}", "\\between", true);
    defineSymbolM!(math, ams, rel, "\u{22d4}", "\\pitchfork", true);
    defineSymbolM!(math, ams, rel, "\u{221d}", "\\varpropto", false);
    defineSymbolM!(math, ams, rel, "\u{25c0}", "\\blacktriangleleft", false);
    defineSymbolM!(math, ams, rel, "\u{2234}", "\\therefore", true);
    defineSymbolM!(math, ams, rel, "\u{220d}", "\\backepsilon", false);
    defineSymbolM!(math, ams, rel, "\u{25b6}", "\\blacktriangleright", false);
    defineSymbolM!(math, ams, rel, "\u{2235}", "\\because", true);
    defineSymbolM!(math, ams, rel, "\u{22d8}", "\\llless", false);
    defineSymbolM!(math, ams, rel, "\u{22d9}", "\\gggtr", false);
    defineSymbolM!(math, ams, bin, "\u{22b2}", "\\lhd", false);
    defineSymbolM!(math, ams, bin, "\u{22b3}", "\\rhd", false);
    defineSymbolM!(math, ams, rel, "\u{2242}", "\\eqsim", true);
    defineSymbolM!(math, main, rel, "\u{22c8}", "\\Join", false);
    defineSymbolM!(math, ams, rel, "\u{2251}", "\\Doteq", true);
    defineSymbolM!(math, ams, bin, "\u{2214}", "\\dotplus", true);
    defineSymbolM!(math, ams, bin, "\u{2216}", "\\smallsetminus", false);
    defineSymbolM!(math, ams, bin, "\u{22d2}", "\\Cap", true);
    defineSymbolM!(math, ams, bin, "\u{22d3}", "\\Cup", true);
    defineSymbolM!(math, ams, bin, "\u{2a5e}", "\\doublebarwedge", true);
    defineSymbolM!(math, ams, bin, "\u{229f}", "\\boxminus", true);
    defineSymbolM!(math, ams, bin, "\u{229e}", "\\boxplus", true);
    defineSymbolM!(math, ams, bin, "\u{22c7}", "\\divideontimes", true);
    defineSymbolM!(math, ams, bin, "\u{22c9}", "\\ltimes", true);
    defineSymbolM!(math, ams, bin, "\u{22ca}", "\\rtimes", true);
    defineSymbolM!(math, ams, bin, "\u{22cb}", "\\leftthreetimes", true);
    defineSymbolM!(math, ams, bin, "\u{22cc}", "\\rightthreetimes", true);
    defineSymbolM!(math, ams, bin, "\u{22cf}", "\\curlywedge", true);
    defineSymbolM!(math, ams, bin, "\u{22ce}", "\\curlyvee", true);
    defineSymbolM!(math, ams, bin, "\u{229d}", "\\circleddash", true);
    defineSymbolM!(math, ams, bin, "\u{229b}", "\\circledast", true);
    defineSymbolM!(math, ams, bin, "\u{22c5}", "\\centerdot", false);
    defineSymbolM!(math, ams, bin, "\u{22ba}", "\\intercal", true);
    defineSymbolM!(math, ams, bin, "\u{22d2}", "\\doublecap", false);
    defineSymbolM!(math, ams, bin, "\u{22d3}", "\\doublecup", false);
    defineSymbolM!(math, ams, bin, "\u{22a0}", "\\boxtimes", true);
    defineSymbolM!(math, ams, rel, "\u{21e2}", "\\dashrightarrow", true);
    defineSymbolM!(math, ams, rel, "\u{21e0}", "\\dashleftarrow", true);
    defineSymbolM!(math, ams, rel, "\u{21c7}", "\\leftleftarrows", true);
    defineSymbolM!(math, ams, rel, "\u{21c6}", "\\leftrightarrows", true);
    defineSymbolM!(math, ams, rel, "\u{21da}", "\\Lleftarrow", true);
    defineSymbolM!(math, ams, rel, "\u{219e}", "\\twoheadleftarrow", true);
    defineSymbolM!(math, ams, rel, "\u{21a2}", "\\leftarrowtail", true);
    defineSymbolM!(math, ams, rel, "\u{21ab}", "\\looparrowleft", true);
    defineSymbolM!(math, ams, rel, "\u{21cb}", "\\leftrightharpoons", true);
    defineSymbolM!(math, ams, rel, "\u{21b6}", "\\curvearrowleft", true);
    defineSymbolM!(math, ams, rel, "\u{21ba}", "\\circlearrowleft", true);
    defineSymbolM!(math, ams, rel, "\u{21b0}", "\\Lsh", true);
    defineSymbolM!(math, ams, rel, "\u{21c8}", "\\upuparrows", true);
    defineSymbolM!(math, ams, rel, "\u{21bf}", "\\upharpoonleft", true);
    defineSymbolM!(math, ams, rel, "\u{21c3}", "\\downharpoonleft", true);
    defineSymbolM!(math, main, rel, "\u{22b6}", "\\origof", true);
    defineSymbolM!(math, main, rel, "\u{22b7}", "\\imageof", true);
    defineSymbolM!(math, ams, rel, "\u{22b8}", "\\multimap", true);
    defineSymbolM!(math, ams, rel, "\u{21ad}", "\\leftrightsquigarrow", true);
    defineSymbolM!(math, ams, rel, "\u{21c9}", "\\rightrightarrows", true);
    defineSymbolM!(math, ams, rel, "\u{21c4}", "\\rightleftarrows", true);
    defineSymbolM!(math, ams, rel, "\u{21a0}", "\\twoheadrightarrow", true);
    defineSymbolM!(math, ams, rel, "\u{21a3}", "\\rightarrowtail", true);
    defineSymbolM!(math, ams, rel, "\u{21ac}", "\\looparrowright", true);
    defineSymbolM!(math, ams, rel, "\u{21b7}", "\\curvearrowright", true);
    defineSymbolM!(math, ams, rel, "\u{21bb}", "\\circlearrowright", true);
    defineSymbolM!(math, ams, rel, "\u{21b1}", "\\Rsh", true);
    defineSymbolM!(math, ams, rel, "\u{21ca}", "\\downdownarrows", true);
    defineSymbolM!(math, ams, rel, "\u{21be}", "\\upharpoonright", true);
    defineSymbolM!(math, ams, rel, "\u{21c2}", "\\downharpoonright", true);
    defineSymbolM!(math, ams, rel, "\u{21dd}", "\\rightsquigarrow", true);
    defineSymbolM!(math, ams, rel, "\u{21dd}", "\\leadsto", false);
    defineSymbolM!(math, ams, rel, "\u{21db}", "\\Rrightarrow", true);
    defineSymbolM!(math, ams, rel, "\u{21be}", "\\restriction", false);
    defineSymbolM!(math, main, textord, "\u{2018}", "`", false);
    defineSymbolM!(math, main, textord, "$", "\\$", false);
    defineSymbolM!(math, main, textord, "%", "\\%", false);
    defineSymbolM!(math, main, textord, "_", "\\_", false);
    defineSymbolM!(math, main, textord, "\u{2220}", "\\angle", true);
    defineSymbolM!(math, main, textord, "\u{221e}", "\\infty", true);
    defineSymbolM!(math, main, textord, "\u{2032}", "\\prime", false);
    defineSymbolM!(math, main, textord, "\u{25b3}", "\\triangle", false);
    defineSymbolM!(math, main, textord, "\u{0393}", "\\Gamma", true);
    defineSymbolM!(math, main, textord, "\u{0394}", "\\Delta", true);
    defineSymbolM!(math, main, textord, "\u{0398}", "\\Theta", true);
    defineSymbolM!(math, main, textord, "\u{039b}", "\\Lambda", true);
    defineSymbolM!(math, main, textord, "\u{039e}", "\\Xi", true);
    defineSymbolM!(math, main, textord, "\u{03a0}", "\\Pi", true);
    defineSymbolM!(math, main, textord, "\u{03a3}", "\\Sigma", true);
    defineSymbolM!(math, main, textord, "\u{03a5}", "\\Upsilon", true);
    defineSymbolM!(math, main, textord, "\u{03a6}", "\\Phi", true);
    defineSymbolM!(math, main, textord, "\u{03a8}", "\\Psi", true);
    defineSymbolM!(math, main, textord, "\u{03a9}", "\\Omega", true);
    defineSymbolM!(math, main, textord, "A", "\u{0391}", false);
    defineSymbolM!(math, main, textord, "B", "\u{0392}", false);
    defineSymbolM!(math, main, textord, "E", "\u{0395}", false);
    defineSymbolM!(math, main, textord, "Z", "\u{0396}", false);
    defineSymbolM!(math, main, textord, "H", "\u{0397}", false);
    defineSymbolM!(math, main, textord, "I", "\u{0399}", false);
    defineSymbolM!(math, main, textord, "K", "\u{039A}", false);
    defineSymbolM!(math, main, textord, "M", "\u{039C}", false);
    defineSymbolM!(math, main, textord, "N", "\u{039D}", false);
    defineSymbolM!(math, main, textord, "O", "\u{039F}", false);
    defineSymbolM!(math, main, textord, "P", "\u{03A1}", false);
    defineSymbolM!(math, main, textord, "T", "\u{03A4}", false);
    defineSymbolM!(math, main, textord, "X", "\u{03A7}", false);
    defineSymbolM!(math, main, textord, "\u{00ac}", "\\neg", true);
    defineSymbolM!(math, main, textord, "\u{00ac}", "\\lnot", false);
    defineSymbolM!(math, main, textord, "\u{22a4}", "\\top", false);
    defineSymbolM!(math, main, textord, "\u{22a5}", "\\bot", false);
    defineSymbolM!(math, main, textord, "\u{2205}", "\\emptyset", false);
    defineSymbolM!(math, ams, textord, "\u{2205}", "\\varnothing", false);
    defineSymbolM!(math, main, mathord, "\u{03b1}", "\\alpha", true);
    defineSymbolM!(math, main, mathord, "\u{03b2}", "\\beta", true);
    defineSymbolM!(math, main, mathord, "\u{03b3}", "\\gamma", true);
    defineSymbolM!(math, main, mathord, "\u{03b4}", "\\delta", true);
    defineSymbolM!(math, main, mathord, "\u{03f5}", "\\epsilon", true);
    defineSymbolM!(math, main, mathord, "\u{03b6}", "\\zeta", true);
    defineSymbolM!(math, main, mathord, "\u{03b7}", "\\eta", true);
    defineSymbolM!(math, main, mathord, "\u{03b8}", "\\theta", true);
    defineSymbolM!(math, main, mathord, "\u{03b9}", "\\iota", true);
    defineSymbolM!(math, main, mathord, "\u{03ba}", "\\kappa", true);
    defineSymbolM!(math, main, mathord, "\u{03bb}", "\\lambda", true);
    defineSymbolM!(math, main, mathord, "\u{03bc}", "\\mu", true);
    defineSymbolM!(math, main, mathord, "\u{03bd}", "\\nu", true);
    defineSymbolM!(math, main, mathord, "\u{03be}", "\\xi", true);
    defineSymbolM!(math, main, mathord, "\u{03bf}", "\\omicron", true);
    defineSymbolM!(math, main, mathord, "\u{03c0}", "\\pi", true);
    defineSymbolM!(math, main, mathord, "\u{03c1}", "\\rho", true);
    defineSymbolM!(math, main, mathord, "\u{03c3}", "\\sigma", true);
    defineSymbolM!(math, main, mathord, "\u{03c4}", "\\tau", true);
    defineSymbolM!(math, main, mathord, "\u{03c5}", "\\upsilon", true);
    defineSymbolM!(math, main, mathord, "\u{03d5}", "\\phi", true);
    defineSymbolM!(math, main, mathord, "\u{03c7}", "\\chi", true);
    defineSymbolM!(math, main, mathord, "\u{03c8}", "\\psi", true);
    defineSymbolM!(math, main, mathord, "\u{03c9}", "\\omega", true);
    defineSymbolM!(math, main, mathord, "\u{03b5}", "\\varepsilon", true);
    defineSymbolM!(math, main, mathord, "\u{03d1}", "\\vartheta", true);
    defineSymbolM!(math, main, mathord, "\u{03d6}", "\\varpi", true);
    defineSymbolM!(math, main, mathord, "\u{03f1}", "\\varrho", true);
    defineSymbolM!(math, main, mathord, "\u{03c2}", "\\varsigma", true);
    defineSymbolM!(math, main, mathord, "\u{03c6}", "\\varphi", true);
    defineSymbolM!(math, main, bin, "\u{2217}", "*", true);
    defineSymbolM!(math, main, bin, "+", "+", false);
    defineSymbolM!(math, main, bin, "\u{2212}", "-", true);
    defineSymbolM!(math, main, bin, "\u{22c5}", "\\cdot", true);
    defineSymbolM!(math, main, bin, "\u{2218}", "\\circ", true);
    defineSymbolM!(math, main, bin, "\u{00f7}", "\\div", true);
    defineSymbolM!(math, main, bin, "\u{00b1}", "\\pm", true);
    defineSymbolM!(math, main, bin, "\u{00d7}", "\\times", true);
    defineSymbolM!(math, main, bin, "\u{2229}", "\\cap", true);
    defineSymbolM!(math, main, bin, "\u{222a}", "\\cup", true);
    defineSymbolM!(math, main, bin, "\u{2216}", "\\setminus", true);
    defineSymbolM!(math, main, bin, "\u{2227}", "\\land", false);
    defineSymbolM!(math, main, bin, "\u{2228}", "\\lor", false);
    defineSymbolM!(math, main, bin, "\u{2227}", "\\wedge", true);
    defineSymbolM!(math, main, bin, "\u{2228}", "\\vee", true);
    defineSymbolM!(math, main, textord, "\u{221a}", "\\surd", false);
    defineSymbolM!(math, main, open, "\u{27e8}", "\\langle", true);
    defineSymbolM!(math, main, open, "\u{2223}", "\\lvert", false);
    defineSymbolM!(math, main, open, "\u{2225}", "\\lVert", false);
    defineSymbolM!(math, main, close, "?", "?", false);
    defineSymbolM!(math, main, close, "!", "!", false);
    defineSymbolM!(math, main, close, "\u{27e9}", "\\rangle", true);
    defineSymbolM!(math, main, close, "\u{2223}", "\\rvert", false);
    defineSymbolM!(math, main, close, "\u{2225}", "\\rVert", false);
    defineSymbolM!(math, main, rel, "=", "=", false);
    defineSymbolM!(math, main, rel, ":", ":", false);
    defineSymbolM!(math, main, rel, "\u{2248}", "\\approx", true);
    defineSymbolM!(math, main, rel, "\u{2245}", "\\cong", true);
    defineSymbolM!(math, main, rel, "\u{2265}", "\\ge", false);
    defineSymbolM!(math, main, rel, "\u{2265}", "\\geq", true);
    defineSymbolM!(math, main, rel, "\u{2190}", "\\gets", false);
    defineSymbolM!(math, main, rel, ">", "\\gt", true);
    defineSymbolM!(math, main, rel, "\u{2208}", "\\in", true);
    defineSymbolM!(math, main, rel, "\u{e020}", "\\@not", false);
    defineSymbolM!(math, main, rel, "\u{2282}", "\\subset", true);
    defineSymbolM!(math, main, rel, "\u{2283}", "\\supset", true);
    defineSymbolM!(math, main, rel, "\u{2286}", "\\subseteq", true);
    defineSymbolM!(math, main, rel, "\u{2287}", "\\supseteq", true);
    defineSymbolM!(math, ams, rel, "\u{2288}", "\\nsubseteq", true);
    defineSymbolM!(math, ams, rel, "\u{2289}", "\\nsupseteq", true);
    defineSymbolM!(math, main, rel, "\u{22a8}", "\\models", false);
    defineSymbolM!(math, main, rel, "\u{2190}", "\\leftarrow", true);
    defineSymbolM!(math, main, rel, "\u{2264}", "\\le", false);
    defineSymbolM!(math, main, rel, "\u{2264}", "\\leq", true);
    defineSymbolM!(math, main, rel, "<", "\\lt", true);
    defineSymbolM!(math, main, rel, "\u{2192}", "\\rightarrow", true);
    defineSymbolM!(math, main, rel, "\u{2192}", "\\to", false);
    defineSymbolM!(math, ams, rel, "\u{2271}", "\\ngeq", true);
    defineSymbolM!(math, ams, rel, "\u{2270}", "\\nleq", true);
    defineSymbolM!(math, main, spacing, "\u{00a0}", "\\ ", false);
    defineSymbolM!(math, main, spacing, "\u{00a0}", "\\space", false);
    defineSymbolM!(math, main, spacing, "\u{00a0}", "\\nobreakspace", false);
    defineSymbolM!(math, main, spacing, None, "\\nobreak", false);
    defineSymbolM!(math, main, spacing, None, "\\allowbreak", false);
    defineSymbolM!(math, main, punct, ",", ",", false);
    defineSymbolM!(math, main, punct, ";", ";", false);
    defineSymbolM!(math, ams, bin, "\u{22bc}", "\\barwedge", true);
    defineSymbolM!(math, ams, bin, "\u{22bb}", "\\veebar", true);
    defineSymbolM!(math, main, bin, "\u{2299}", "\\odot", true);
    defineSymbolM!(math, main, bin, "\u{2295}", "\\oplus", true);
    defineSymbolM!(math, main, bin, "\u{2297}", "\\otimes", true);
    defineSymbolM!(math, main, textord, "\u{2202}", "\\partial", true);
    defineSymbolM!(math, main, bin, "\u{2298}", "\\oslash", true);
    defineSymbolM!(math, ams, bin, "\u{229a}", "\\circledcirc", true);
    defineSymbolM!(math, ams, bin, "\u{22a1}", "\\boxdot", true);
    defineSymbolM!(math, main, bin, "\u{25b3}", "\\bigtriangleup", false);
    defineSymbolM!(math, main, bin, "\u{25bd}", "\\bigtriangledown", false);
    defineSymbolM!(math, main, bin, "\u{2020}", "\\dagger", false);
    defineSymbolM!(math, main, bin, "\u{22c4}", "\\diamond", false);
    defineSymbolM!(math, main, bin, "\u{22c6}", "\\star", false);
    defineSymbolM!(math, main, bin, "\u{25c3}", "\\triangleleft", false);
    defineSymbolM!(math, main, bin, "\u{25b9}", "\\triangleright", false);
    defineSymbolM!(math, main, open, "{", "\\{", false);
    defineSymbolM!(math, main, close, "}", "\\}", false);
    defineSymbolM!(math, main, open, "{", "\\lbrace", false);
    defineSymbolM!(math, main, close, "}", "\\rbrace", false);
    defineSymbolM!(math, main, open, "[", "\\lbrack", true);
    defineSymbolM!(math, main, close, "]", "\\rbrack", true);
    defineSymbolM!(math, main, open, "(", "\\lparen", true);
    defineSymbolM!(math, main, close, ")", "\\rparen", true);
    defineSymbolM!(math, main, open, "\u{230a}", "\\lfloor", true);
    defineSymbolM!(math, main, close, "\u{230b}", "\\rfloor", true);
    defineSymbolM!(math, main, open, "\u{2308}", "\\lceil", true);
    defineSymbolM!(math, main, close, "\u{2309}", "\\rceil", true);
    defineSymbolM!(math, main, textord, "\\", "\\backslash", false);
    defineSymbolM!(math, main, textord, "\u{2223}", "|", false);
    defineSymbolM!(math, main, textord, "\u{2223}", "\\vert", false);
    defineSymbolM!(math, main, textord, "\u{2225}", "\\|", false);
    defineSymbolM!(math, main, textord, "\u{2225}", "\\Vert", false);
    defineSymbolM!(math, main, rel, "\u{2191}", "\\uparrow", true);
    defineSymbolM!(math, main, rel, "\u{21d1}", "\\Uparrow", true);
    defineSymbolM!(math, main, rel, "\u{2193}", "\\downarrow", true);
    defineSymbolM!(math, main, rel, "\u{21d3}", "\\Downarrow", true);
    defineSymbolM!(math, main, rel, "\u{2195}", "\\updownarrow", true);
    defineSymbolM!(math, main, rel, "\u{21d5}", "\\Updownarrow", true);
    defineSymbolM!(math, main, op, "\u{2210}", "\\coprod", false);
    defineSymbolM!(math, main, op, "\u{22c1}", "\\bigvee", false);
    defineSymbolM!(math, main, op, "\u{22c0}", "\\bigwedge", false);
    defineSymbolM!(math, main, op, "\u{2a04}", "\\biguplus", false);
    defineSymbolM!(math, main, op, "\u{22c2}", "\\bigcap", false);
    defineSymbolM!(math, main, op, "\u{22c3}", "\\bigcup", false);
    defineSymbolM!(math, main, op, "\u{222b}", "\\int", false);
    defineSymbolM!(math, main, op, "\u{222b}", "\\intop", false);
    defineSymbolM!(math, main, op, "\u{222c}", "\\iint", false);
    defineSymbolM!(math, main, op, "\u{222d}", "\\iiint", false);
    defineSymbolM!(math, main, op, "\u{220f}", "\\prod", false);
    defineSymbolM!(math, main, op, "\u{2211}", "\\sum", false);
    defineSymbolM!(math, main, op, "\u{2a02}", "\\bigotimes", false);
    defineSymbolM!(math, main, op, "\u{2a01}", "\\bigoplus", false);
    defineSymbolM!(math, main, op, "\u{2a00}", "\\bigodot", false);
    defineSymbolM!(math, main, op, "\u{222e}", "\\oint", false);
    defineSymbolM!(math, main, op, "\u{222f}", "\\oiint", false);
    defineSymbolM!(math, main, op, "\u{2230}", "\\oiiint", false);
    defineSymbolM!(math, main, op, "\u{2a06}", "\\bigsqcup", false);
    defineSymbolM!(math, main, op, "\u{222b}", "\\smallint", false);
    defineSymbolM!(math, main, inner, "\u{2026}", "\\mathellipsis", false);
    defineSymbolM!(math, main, inner, "\u{2026}", "\\ldots", true);
    defineSymbolM!(math, main, inner, "\u{22ef}", "\\@cdots", true);
    defineSymbolM!(math, main, inner, "\u{22f1}", "\\ddots", true);
    defineSymbolM!(math, main, textord, "\u{22ee}", "\\varvdots", false);
    defineSymbolM!(math, main, accent, "\u{02ca}", "\\acute", false);
    defineSymbolM!(math, main, accent, "\u{02cb}", "\\grave", false);
    defineSymbolM!(math, main, accent, "\u{00a8}", "\\ddot", false);
    defineSymbolM!(math, main, accent, "\u{007e}", "\\tilde", false);
    defineSymbolM!(math, main, accent, "\u{02c9}", "\\bar", false);
    defineSymbolM!(math, main, accent, "\u{02d8}", "\\breve", false);
    defineSymbolM!(math, main, accent, "\u{02c7}", "\\check", false);
    defineSymbolM!(math, main, accent, "\u{005e}", "\\hat", false);
    defineSymbolM!(math, main, accent, "\u{20d7}", "\\vec", false);
    defineSymbolM!(math, main, accent, "\u{02d9}", "\\dot", false);
    defineSymbolM!(math, main, accent, "\u{02da}", "\\mathring", false);
    defineSymbolM!(math, main, mathord, "\u{e131}", "\\@imath", false);
    defineSymbolM!(math, main, mathord, "\u{e237}", "\\@jmath", false);
    defineSymbolM!(math, main, textord, "\u{0131}", "\u{0131}", false);
    defineSymbolM!(math, main, textord, "\u{0237}", "\u{0237}", false);
    defineSymbolM!(math, main, textord, "\u{00b0}", "\\degree", true);
    defineSymbolM!(math, main, textord, "\u{00a3}", "\\pounds", false);
    defineSymbolM!(math, main, textord, "\u{00a3}", "\\mathsterling", true);
    defineSymbolM!(math, ams, textord, "\u{2720}", "\\maltese", false);

    for ch in String::from("0123456789/@.\"").chars() {
        defineSymbolM!(math, main, textord, ch, ch, false);
    }
    defineSymbolM!(math, ams, textord, "C", "\u{2102}", false);
    defineSymbolM!(math, ams, textord, "H", "\u{210D}", false);
    defineSymbolM!(math, ams, textord, "N", "\u{2115}", false);
    defineSymbolM!(math, ams, textord, "P", "\u{2119}", false);
    defineSymbolM!(math, ams, textord, "Q", "\u{211A}", false);
    defineSymbolM!(math, ams, textord, "R", "\u{211D}", false);
    defineSymbolM!(math, ams, textord, "Z", "\u{2124}", false);
    defineSymbolM!(math, main, mathord, "h", "\u{210E}", false);
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    for ch in letters.chars() {
        defineSymbolM!(math, main, mathord, ch, ch);
    }
    let mut i: u16 = 0;
    for ch in letters.chars() {
        let mut wide_char = code_to_str(0xD835, 0xDC00 + i);
        defineSymbolM!(math, main, mathord, ch, wide_char, false);
        wide_char = code_to_str(0xD835, 0xDC34 + i);
        defineSymbolM!(math, main, mathord, ch, wide_char, false);
        wide_char = code_to_str(0xD835, 0xDC68 + i);
        defineSymbolM!(math, main, mathord, ch, wide_char, false);
        wide_char = code_to_str(0xD835, 0xDD04 + i);
        defineSymbolM!(math, main, mathord, ch, wide_char, false);
        wide_char = code_to_str(0xD835, 0xDDA0 + i);
        defineSymbolM!(math, main, mathord, ch, wide_char, false);
        wide_char = code_to_str(0xD835, 0xDDD4 + i);
        defineSymbolM!(math, main, mathord, ch, wide_char, false);
        wide_char = code_to_str(0xD835, 0xDE08 + i);
        defineSymbolM!(math, main, mathord, ch, wide_char, false);
        wide_char = code_to_str(0xD835, 0xDE70 + i);
        defineSymbolM!(math, main, mathord, ch, wide_char, false);
        if i < 26 {
            wide_char = code_to_str(0xD835, 0xDD38 + i);
            defineSymbolM!(math, main, mathord, ch, wide_char, false);
            wide_char = code_to_str(0xD835, 0xDC9C + i);
            defineSymbolM!(math, main, mathord, ch, wide_char, false);
        }
        i += 1;
    }

    // "k" is the only double struck lower case letter in the KaTeX fonts.
    // k double struck
    defineSymbolM!(math, main, mathord, "k", code_to_str(0xD835, 0xDD5C), false);
    i = 0;
    for ch in '0'..'9' {
        let mut wide_char = code_to_str(0xD835, 0xDFCE + i); // 0-9 bold
        defineSymbolM!(math, main, mathord, ch, wide_char, false);
        wide_char = code_to_str(0xD835, 0xDFE2 + i); // 0-9 sans serif
        defineSymbolM!(math, main, mathord, ch, wide_char, false);
        wide_char = code_to_str(0xD835, 0xDFEC + i); // 0-9 bold sans
        defineSymbolM!(math, main, mathord, ch, wide_char, false);
        wide_char = code_to_str(0xD835, 0xDFF6 + i); // 0-9 monospace
        defineSymbolM!(math, main, mathord, ch, wide_char, false);
        i += 1;
    }

    for ch in String::from("\u{00d0}\u{00de}\u{00fe}").chars() {
        defineSymbolM!(math, main, mathord, ch, ch, false);
    }
    math
}
