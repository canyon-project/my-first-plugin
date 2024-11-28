use swc_core::ecma::{
    ast::Program,
    transforms::testing::test_inline,
    visit::{as_folder, FoldWith, VisitMut},
};
use swc_core::ecma::ast::{op, BinExpr};
use swc_core::ecma::visit::VisitMutWith;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_bin_expr(&mut self, e: &mut BinExpr) {
        e.visit_mut_children_with(self);

        if e.op == op!("==") {
            e.op = op!("===");
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    boo,
    // 输入代码
    r#"123 == '123';"#,
    // 经过插件转换后的输出代码
    r#"123 === '123';"#
);