@0xbcc55cbe2b79513a;

interface RampInterface {
    prepare @0 (key:Text, value:Text, dependencies:List(Text), timestamp:Int64);
    commit @1 (timestamp:Int64);
    get @2 (key:Text) -> (value:Text, timestamp:Int64, dependencies:List(Text));
    getVersion @3 (key:Text, timestamp:Int64) -> (value:Text, timestamp:Int64, dependencies:List(Text));
}
