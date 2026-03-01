mod accent;
mod accentunder;
mod arrow;
mod cr;
mod delimsizing;
mod enclose;
mod environment;
mod font;
mod genfrac;
mod mclass;
mod ordgroup;
mod supsub;
mod symbols_op;
mod symbols_ord;
mod horiz_brace;
mod href;
mod text;
mod hbox;
mod html;
mod sqrt;
mod includegraphics;
mod lap;
mod mathchoice;
mod op;
mod overline;
mod phantom;
mod pmb;
mod raisebox;
mod smash;
mod underline;
mod sizing;
mod kern;
mod verb;
mod vcenter;
mod tag;
mod symbols_spacing;
mod styling;
mod rule;
mod relax;
mod operatorname;
mod math;
mod htmlmathml;
mod char;
mod color;
mod assembleSupSub;
mod def;

use super::public::FunctionDefSpec;
use std::sync::Mutex;
use crate::define::functions::def_spec::vcenter::VCENTER;

lazy_static! {
    pub static ref FUNCS: Mutex<Vec<FunctionDefSpec>> = Mutex::new({
        let x = mclass::MCLASS.lock().unwrap();
        let y = symbols_ord::MATHORD.lock().unwrap();
        let z = symbols_op::ATOM.lock().unwrap();
        //
        let a = supsub::SUPSUB.lock().unwrap();
        let b = symbols_ord::TEXTORD.lock().unwrap();
        let c = accent::ACCENT.lock().unwrap();
        let c2 = accent::ACCENT2.lock().unwrap();
        let d = ordgroup::ORDGROUP.lock().unwrap();
        let e = accentunder::ACCENTUNDER.lock().unwrap();
        let f = arrow::XARROW.lock().unwrap();
        let g = cr::CR.lock().unwrap();
        let h1 = enclose::COLOR_BOX.lock().unwrap();
        let h2 = enclose::FCOLOR_BOX.lock().unwrap();
        let h3 = enclose::FBOX.lock().unwrap();
        let h4 = enclose::CANCEL.lock().unwrap();
        let h5 = enclose::ANGL.lock().unwrap();
        let i1 = genfrac::FRAC.lock().unwrap();
        let i2 = genfrac::OCABB.lock().unwrap();
        let j = horiz_brace::HORIZBRACE.lock().unwrap();
        let k1 = href::HREF.lock().unwrap();
        let k2 = href::URL.lock().unwrap();
        let l = text::TEXT.lock().unwrap();
        let m = hbox::HBOX.lock().unwrap();
        let n = html::HTML_SPEC.lock().unwrap();
        let o = sqrt::SQRT.lock().unwrap();
        let p = includegraphics::INCLUDE_GRAPHICS.lock().unwrap();
        let q = lap::LAP.lock().unwrap();
        let r = mathchoice::MATH_CHOICE.lock().unwrap();
        let s1 = op::OP.lock().unwrap();
        let s2 = op::MATH_OP.lock().unwrap();
        let s3 = op::TRIGNONOMETRIC_OP.lock().unwrap();
        let s4 = op::GCD_OP.lock().unwrap();
        let s5 = op::INT_OP.lock().unwrap();
        let t = overline::OVERLINE.lock().unwrap();
        let u1 = phantom::PHANTOM.lock().unwrap();
        let u2 = phantom::HPHANTOM.lock().unwrap();
        let u3 = phantom::VPHANTOM.lock().unwrap();
        let v = pmb::PMB.lock().unwrap();
        let w = raisebox::RAISEBOX.lock().unwrap();
        let aa = smash::SMASH.lock().unwrap();
        let ab = underline::UNDERLINE.lock().unwrap();
        let ac = environment::ENV.lock().unwrap();
        let ad = font::FONT1.lock().unwrap();
        let ae = font::FONT2.lock().unwrap();
        let af = font::FONT3.lock().unwrap();
        let ba = delimsizing::BIG.lock().unwrap();
        let bb = delimsizing::LEFTRIGHT.lock().unwrap();
        let bc = delimsizing::LEFTRIGHT_RIGHT.lock().unwrap();
        let bd = delimsizing::MIDDLE.lock().unwrap();

        let ca = sizing::SIZING.lock().unwrap();

        let da = kern::KERN.lock().unwrap();

        let ea = verb::VERB.lock().unwrap();

        let fa = vcenter::VCENTER.lock().unwrap();

        let ga = tag::TAG.lock().unwrap();
        let ha = styling::STYLING.lock().unwrap();
        let hb = math::STYLING.lock().unwrap();
        let hc = math::TEXT.lock().unwrap();

        let ia = rule::RULE.lock().unwrap();
        let ib = relax::RULE.lock().unwrap();

        let ja = color::COLOR.lock().unwrap();
        let jb = color::COLOR2.lock().unwrap();

        let ka = htmlmathml::HTML_MATHML.lock().unwrap();

        let la = operatorname::ORDGROUP.lock().unwrap();

        let ma = symbols_spacing::TAG.lock().unwrap();

        let na = def::INTERNAL.lock().unwrap();
        let nb = def::INTERNAL2.lock().unwrap();
        let nc = def::INTERNAL3.lock().unwrap();
        let nd = def::INTERNAL4.lock().unwrap();

        let res = vec![
            x.clone(),
            y.clone(),
            z.clone(),
            //
            a.clone(),
            b.clone(),
            c.clone(),
            c2.clone(),
            d.clone(),
            e.clone(),
            f.clone(),
            g.clone(),
            h1.clone(),
            h2.clone(),
            h3.clone(),
            h4.clone(),
            h5.clone(),
            i1.clone(),
            i2.clone(),
            j.clone(),
            k1.clone(),
            k2.clone(),
            l.clone(),
            m.clone(),
            n.clone(),
            o.clone(),
            p.clone(),
            q.clone(),
            r.clone(),
            s1.clone(),
            s2.clone(),
            s3.clone(),
            s4.clone(),
            s5.clone(),
            t.clone(),
            u1.clone(),
            u2.clone(),
            u3.clone(),
            v.clone(),
            w.clone(),
            aa.clone(),
            ab.clone(),
            ac.clone(),
            ad.clone(),
            ae.clone(),
            af.clone(),
            ba.clone(),
            bc.clone(),
            bb.clone(),
            bd.clone(),
            ca.clone(),
            da.clone(),
            ea.clone(),
            fa.clone(),
            ga.clone(),
            ha.clone(),
            hb.clone(),
            hc.clone(),
            ia.clone(),
            ib.clone(),
            ja.clone(),
            jb.clone(),
            ka.clone(),
            la.clone(),
            ma.clone(),
            na.clone(),
            nb.clone(),
            nc.clone(),
            nd.clone()
        ];
        res
    });
}
