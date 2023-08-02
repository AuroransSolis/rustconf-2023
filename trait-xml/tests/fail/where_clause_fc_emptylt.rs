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
                <lifetime></lifetime>
            </for-clause>
        </where>
    </trait>
}

fn main() {}
