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
                <type><Bar as Iterator>::Item</type>
                <type-bound></type-bound>
            </type-clause>
        </where>
    </trait>
}

fn main() {}
