trait_xml::trait_xml! {
    <trait>
        <name>LifetimeClauseTest</name>
        <bounds>
            <lifetime>
                <name>'foo</name>
            </lifetime>
            <lifetime>
                <name>'bar</name>
            </lifetime>
            <lifetime>
                <name>'baz</name>
            </lifetime>
        </bounds>
        <where>
            <lifetime-clause>
                <lifetime>'foo</lifetime>
                <lifetime-bound>'bar</lifetime-bound>
                <lifetime-bound>'baz</lifetime-bound>
            </lifetime-clause>
            <lifetime-clause>
                <lifetime>'bar</lifetime>
                <lifetime-bound>'baz</lifetime-bound>
            </lifetime-clause>
        </where>
    </trait>
}

fn main() {}
