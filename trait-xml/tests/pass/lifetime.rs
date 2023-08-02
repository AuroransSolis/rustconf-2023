trait_xml::trait_xml! {
    <trait>
        <name>Foo</name>
        <bounds>
            <lifetime>
                <name>'bar</name>
            </lifetime>
            <lifetime>
                <name>'baz</name>
                <lifetime-bound>'bar</lifetime-bound>
            </lifetime>
        </bounds>
    </trait>
}

fn main() {}
