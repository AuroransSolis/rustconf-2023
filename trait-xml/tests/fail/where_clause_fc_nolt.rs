trait_xml::trait_xml! {
    <trait>
        <name>Foo</name>
        <bounds>
            <type>
                <name>Bar</name>
            </type>
        </bounds>
        <where>
            <for-clause>
                <type-clause>
                    <type>Bar</type>
                    <type-bound>std::ops::Fn(&'baz u8)</type-bound>
                </type-clause>
            </for-clause>
        </where>
    </trait>
}

fn main() {}
