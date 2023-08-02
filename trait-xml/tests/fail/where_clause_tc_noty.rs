trait_xml::trait_xml! {
    <trait>
        <name>Foo</name>
        <bounds>
            <type>
                <name>Bar</name>
                <type-bound>Iterator</type-bound>
            </type>
        </bounds>
        <where>
            <type-clause>
                <type-bound>Clone</type-bound>
            </type-clause>
        </where>
    </trait>
}

fn main() {}
