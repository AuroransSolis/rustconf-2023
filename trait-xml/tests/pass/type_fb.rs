trait_xml::trait_xml! {
    <trait>
        <name>Foo</name>
        <bounds>
            <type>
                <name>Bar</name>
                <for-bound>
                    <lifetime>'baz</lifetime>
                    <type-bound>std::ops::Fn(&'baz u8)</type-bound>
                </for-bound>
            </type>
        </bounds>
    </trait>
}

fn main() {}
