trait_xml::trait_xml! {
    <trait>
        <name>Foo</name>
        <bounds>
            <lifetime>
                <name>'bar</name>
            </lifetime>
            <type>
                <name>Baz</name>
                <lifetime-bound></lifetime-bound>
            </type>
        </bounds>
    </trait>
}

fn main() {}
