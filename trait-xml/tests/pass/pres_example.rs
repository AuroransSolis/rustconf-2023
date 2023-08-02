trait_xml::trait_xml! {
    <trait>
        <name>Foo</name>
        <vis>pub</vis>
        <bounds>
            <const>
                <name>BAR</name>
                <type>usize</type>
            </const>
            <req>Baz</req>
        </bounds>
        <assoctype>
            <name>Baq</name>
            <bounds>
                <req>Qux</req>
            </bounds>
        </assoctype>
        <assocconst>
		<name>QUUX</name>
		<type>Self::Baq</type>
        </assocconst>
        <assocfn>
            <name>corge</name>
            <gparams>
                <type>
                    <name>Grault</name>
                </type>
                <type>
                    <name>Garply</name>
                </type>
            </gparams>
            <params>
                <param>
                    <name>waldo</name>
                    <type>Grault</type>
                </param>
            </params>
            <ret>Garply</ret>
        </assocfn>
    </trait>
}

pub trait Baz {}

pub trait Qux {}

fn main() {}