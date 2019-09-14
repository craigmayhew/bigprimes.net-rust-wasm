use seed::prelude::*;
use crate::Msg;

mod fermat_utils {
    use seed::prelude::*;
    use crate::Msg;

    pub fn save_as_file(filename:String, filecontent:String) -> seed::dom_types::Node<Msg> {
        let href:String = vec!["data:text/plain;",&filecontent].into_iter().collect();
        a![attrs!{At::Download => &filename, At::Href => &href}, "TXT"]
    }
}

pub fn render() -> seed::dom_types::Node<Msg> {
    
    div![
        h1!["The Fermat Numbers"],
        br![],
        br![],
        br![],
        //todo: replace with rust elements
        El::from_html("<table id=\"tbl\"><tbody><tr><th>&nbsp;</th><th>No.</th><th>Fermat</th><th>Digits</th><th>Prime / Factors</th><th>Download</th></tr><tr><td>&nbsp;</td><td>11</td><td>2<sup>2048</sup>+1</td><td></td><td><a href=\"/cruncher/319489/\">P27567</a> × 974849 × 167988556341760475137 × 3560841906445833920513 × <a href=\"/cruncher/4093/\">P564</a></td><td></td></tr><tr><td>&nbsp;</td><td>10</td><td>2<sup>1024</sup>+1</td><td></td><td>45592577 × 6487031809 × 4659775785220018543264560743076778192897 × <a href=\"/cruncher/1601/\">P252</a></td><td></td></tr><tr><td>&nbsp;</td><td>9</td><td>2<sup>512</sup>+1</td><td>155</td><td>2424833 × 7455602825647884208337395736200454918783366342657 × <a href=\"/cruncher/523/\">P99</a></td><td><a href=\"https://static.bigprimes.net/archive/fermat/F9.txt\">TXT</a></td></tr><tr><td>&nbsp;</td><td>8</td><td>2<sup>256</sup>+1</td><td>78</td><td>1238926361552897 × <a href=\"/cruncher/293/\">P62</a></td><td><a href=\"https://static.bigprimes.net/archive/fermat/F8.txt\">TXT</a></td></tr><tr><td>&nbsp;</td><td>7</td><td>2<sup>128</sup>+1</td><td>39</td><td>59649589127497217 × 5704689200685129054721</td><td><a href=\"https://static.bigprimes.net/archive/fermat/F7.txt\">TXT</a></td></tr><tr><td>&nbsp;</td><td>6</td><td>2<sup>64</sup>+1</td><td>20</td><td>274177 × 67280421310721</td><td><a href=\"https://static.bigprimes.net/archive/fermat/F6.txt\">TXT</a></td></tr><tr><td>&nbsp;</td><td>5</td><td>2<sup>32</sup>+1</td><td>10</td><td><a href=\"/cruncher/641/\">P116</a> × 6700417</td><td><a href=\"https://static.bigprimes.net/archive/fermat/F5.txt\">TXT</a></td></tr><tr><td>&nbsp;</td><td>4</td><td>2<sup>16</sup>+1</td><td>5</td><td><a href=\"/cruncher/65537/\">P6543</a></td><td><a href=\"https://static.bigprimes.net/archive/fermat/F4.txt\">TXT</a></td></tr><tr><td>&nbsp;</td><td>3</td><td>2<sup>8</sup>+1</td><td>3</td><td><a href=\"/cruncher/257/\">P55</a></td><td><a href=\"https://static.bigprimes.net/archive/fermat/F3.txt\">TXT</a></td></tr><tr><td>&nbsp;</td><td>2</td><td>2<sup>4</sup>+1</td><td>2</td><td><a href=\"/cruncher/17/\">P7</a></td><td><a href=\"https://static.bigprimes.net/archive/fermat/F2.txt\">TXT</a></td></tr><tr><td>&nbsp;</td><td>1</td><td>2<sup>2</sup>+1</td><td>1</td><td><a href=\"/cruncher/5/\">P3</a></td><td><a href=\"https://static.bigprimes.net/archive/fermat/F1.txt\">TXT</a></td></tr><tr><td>&nbsp;</td><td>0</td><td>2<sup>1</sup>+1</td><td>1</td><td><a href=\"/cruncher/3/\">P2</a></td><td><a href=\"https://static.bigprimes.net/archive/fermat/F0.txt\">TXT</a></td></tr></tbody></table>"),
    ]
}