trait_xml::trait_xml! {
    <trait>
        <name>Foo</name>
        <bounds>
            <type>
                <name>Bar</name>
                <for-bound>
                    <lifetime>'baz</lifetime>
                </for-bound>
            </type>
        </bounds>
    </trait>
}

fn main() {}
