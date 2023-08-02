trait_xml::trait_xml! {
    <trait>
        <name>Foo</name>
        <bounds>
            <type>
                <name>Bar</name>
                <type-bound>Baz</type-bound>
            </type>
        </bounds>
    </trait>
}

trait Baz {}

fn main() {}
