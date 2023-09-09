// @generated
// generated running `cargo build -F gen-tests`
// test macros are defined in tests/common/mod.rs
mod common;

#[test]
fn markdown_tabs_1() {
    // https://spec.commonmark.org/0.30/#example-1
    test_identical_markdown_events!(r##"	foo	baz		bim"##,r##"    foo	baz		bim"##);
}

#[test]
fn markdown_tabs_2() {
    // https://spec.commonmark.org/0.30/#example-2
    test_identical_markdown_events!(r##"  	foo	baz		bim"##,r##"    foo	baz		bim"##);
}

#[test]
fn markdown_tabs_3() {
    // https://spec.commonmark.org/0.30/#example-3
    test_identical_markdown_events!(r##"    a	a
    ὐ	a"##);
}

#[test]
fn markdown_tabs_4() {
    // https://spec.commonmark.org/0.30/#example-4
    test_identical_markdown_events!(r##"  - foo

	bar"##,r##"- foo

  bar"##);
}

#[test]
fn markdown_tabs_5() {
    // https://spec.commonmark.org/0.30/#example-5
    test_identical_markdown_events!(r##"- foo

		bar"##,r##"- foo

        bar"##);
}

#[test]
fn markdown_tabs_6() {
    // https://spec.commonmark.org/0.30/#example-6
    test_identical_markdown_events!(r##">		foo"##,r##">       foo"##);
}

#[test]
fn markdown_tabs_7() {
    // https://spec.commonmark.org/0.30/#example-7
    test_identical_markdown_events!(r##"-		foo"##,r##"-       foo"##);
}

#[test]
fn markdown_tabs_8() {
    // https://spec.commonmark.org/0.30/#example-8
    test_identical_markdown_events!(r##"    foo
	bar"##,r##"    foo
    bar"##);
}

#[test]
fn markdown_tabs_9() {
    // https://spec.commonmark.org/0.30/#example-9
    test_identical_markdown_events!(r##" - foo
   - bar
	 - baz"##,r##"- foo
  - bar
    - baz"##);
}

#[test]
fn markdown_tabs_10() {
    // https://spec.commonmark.org/0.30/#example-10
    test_identical_markdown_events!(r##"#	Foo"##,r##"# Foo"##);
}

#[test]
fn markdown_tabs_11() {
    // https://spec.commonmark.org/0.30/#example-11
    test_identical_markdown_events!("*\t*\t*\t");
}

#[test]
fn markdown_backslash_escapes_12() {
    // https://spec.commonmark.org/0.30/#example-12
    test_identical_markdown_events!(r##"\!\"\#\$\%\&\'\(\)\*\+\,\-\.\/\:\;\<\=\>\?\@\[\\\]\^\_\`\{\|\}\~"##);
}

#[test]
fn markdown_backslash_escapes_13() {
    // https://spec.commonmark.org/0.30/#example-13
    test_identical_markdown_events!(r##"\	\A\a\ \3\φ\«"##);
}

#[test]
fn markdown_backslash_escapes_14() {
    // https://spec.commonmark.org/0.30/#example-14
    test_identical_markdown_events!(r##"\*not emphasized*
\<br/> not a tag
\[not a link](/foo)
\`not code`
1\. not a list
\* not a list
\# not a heading
\[foo]: /url "not a reference"
\&ouml; not a character entity"##);
}

#[test]
fn markdown_backslash_escapes_15() {
    // https://spec.commonmark.org/0.30/#example-15
    test_identical_markdown_events!(r##"\\*emphasis*"##);
}

#[test]
fn markdown_backslash_escapes_16() {
    // https://spec.commonmark.org/0.30/#example-16
    test_identical_markdown_events!(r##"foo\
bar"##);
}

#[test]
fn markdown_backslash_escapes_17() {
    // https://spec.commonmark.org/0.30/#example-17
    test_identical_markdown_events!(r##"`` \[\` ``"##);
}

#[test]
fn markdown_backslash_escapes_18() {
    // https://spec.commonmark.org/0.30/#example-18
    test_identical_markdown_events!(r##"    \[\]"##);
}

#[test]
fn markdown_backslash_escapes_19() {
    // https://spec.commonmark.org/0.30/#example-19
    test_identical_markdown_events!(r##"~~~
\[\]
~~~"##);
}

#[test]
fn markdown_backslash_escapes_20() {
    // https://spec.commonmark.org/0.30/#example-20
    test_identical_markdown_events!(r##"<http://example.com?find=\*>"##);
}

#[test]
fn markdown_backslash_escapes_21() {
    // https://spec.commonmark.org/0.30/#example-21
    test_identical_markdown_events!(r##"<a href="/bar\/)">"##);
}

#[test]
fn markdown_backslash_escapes_22() {
    // https://spec.commonmark.org/0.30/#example-22
    test_identical_markdown_events!(r##"[foo](/bar\* "ti\*tle")"##,r##"[foo](/bar\* "ti\*tle")"##);
}

#[test]
fn markdown_backslash_escapes_23() {
    // https://spec.commonmark.org/0.30/#example-23
    test_identical_markdown_events!(r##"[foo]

[foo]: /bar\* "ti\*tle""##,r##"[foo]

[foo]: /bar\* "ti\*tle""##);
}

#[test]
fn markdown_backslash_escapes_24() {
    // https://spec.commonmark.org/0.30/#example-24
    test_identical_markdown_events!(r##"``` foo\+bar
foo
```"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_25() {
    // https://spec.commonmark.org/0.30/#example-25
    test_identical_markdown_events!(r##"&nbsp; &amp; &copy; &AElig; &Dcaron;
&frac34; &HilbertSpace; &DifferentialD;
&ClockwiseContourIntegral; &ngE;"##,r##"&nbsp; &amp; &copy; &AElig; &Dcaron;
&frac34; &HilbertSpace; &DifferentialD;
&ClockwiseContourIntegral; &ngE;"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_26() {
    // https://spec.commonmark.org/0.30/#example-26
    test_identical_markdown_events!(r##"&#35; &#1234; &#992; &#0;"##,r##"&#35; &#1234; &#992; &#0;"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_27() {
    // https://spec.commonmark.org/0.30/#example-27
    test_identical_markdown_events!(r##"&#X22; &#XD06; &#xcab;"##,r##"&#X22; &#XD06; &#xcab;"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_28() {
    // https://spec.commonmark.org/0.30/#example-28
    test_identical_markdown_events!(r##"&nbsp &x; &#; &#x;
&#87654321;
&#abcdef0;
&ThisIsNotDefined; &hi?;"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_29() {
    // https://spec.commonmark.org/0.30/#example-29
    test_identical_markdown_events!(r##"&copy"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_30() {
    // https://spec.commonmark.org/0.30/#example-30
    test_identical_markdown_events!(r##"&MadeUpEntity;"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_31() {
    // https://spec.commonmark.org/0.30/#example-31
    test_identical_markdown_events!(r##"<a href="&ouml;&ouml;.html">"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_32() {
    // https://spec.commonmark.org/0.30/#example-32
    test_identical_markdown_events!(r##"[foo](/f&ouml;&ouml; "f&ouml;&ouml;")"##,r##"[foo](/f&ouml;&ouml; "f&ouml;&ouml;")"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_33() {
    // https://spec.commonmark.org/0.30/#example-33
    test_identical_markdown_events!(r##"[foo]

[foo]: /f&ouml;&ouml; "f&ouml;&ouml;""##,r##"[foo]

[foo]: /f&ouml;&ouml; "f&ouml;&ouml;""##);
}

#[test]
fn markdown_entity_and_numeric_character_references_34() {
    // https://spec.commonmark.org/0.30/#example-34
    test_identical_markdown_events!(r##"``` f&ouml;&ouml;
foo
```"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_35() {
    // https://spec.commonmark.org/0.30/#example-35
    test_identical_markdown_events!(r##"`f&ouml;&ouml;`"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_36() {
    // https://spec.commonmark.org/0.30/#example-36
    test_identical_markdown_events!(r##"    f&ouml;f&ouml;"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_37() {
    // https://spec.commonmark.org/0.30/#example-37
    test_identical_markdown_events!(r##"&#42;foo&#42;
*foo*"##,r##"&#42;foo&#42;
*foo*"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_38() {
    // https://spec.commonmark.org/0.30/#example-38
    test_identical_markdown_events!(r##"&#42; foo

* foo"##,r##"&#42; foo

* foo"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_39() {
    // https://spec.commonmark.org/0.30/#example-39
    test_identical_markdown_events!(r##"foo&#10;&#10;bar"##,r##"foo&#10;&#10;bar"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_40() {
    // https://spec.commonmark.org/0.30/#example-40
    test_identical_markdown_events!(r##"&#9;foo"##,r##"&#9;foo"##);
}

#[test]
fn markdown_entity_and_numeric_character_references_41() {
    // https://spec.commonmark.org/0.30/#example-41
    test_identical_markdown_events!(r##"[a](url &quot;tit&quot;)"##,r##"[a](url &quot;tit&quot;)"##);
}

#[test]
fn markdown_precedence_42() {
    // https://spec.commonmark.org/0.30/#example-42
    test_identical_markdown_events!(r##"- `one
- two`"##);
}

#[test]
fn markdown_thematic_breaks_43() {
    // https://spec.commonmark.org/0.30/#example-43
    test_identical_markdown_events!(r##"***
---
___"##);
}

#[test]
fn markdown_thematic_breaks_44() {
    // https://spec.commonmark.org/0.30/#example-44
    test_identical_markdown_events!(r##"+++"##);
}

#[test]
fn markdown_thematic_breaks_45() {
    // https://spec.commonmark.org/0.30/#example-45
    test_identical_markdown_events!(r##"==="##);
}

#[test]
fn markdown_thematic_breaks_46() {
    // https://spec.commonmark.org/0.30/#example-46
    test_identical_markdown_events!(r##"--
**
__"##,r##"--
**
__"##);
}

#[test]
fn markdown_thematic_breaks_47() {
    // https://spec.commonmark.org/0.30/#example-47
    test_identical_markdown_events!(r##" ***
  ***
   ***"##,r##"***
***
***"##);
}

#[test]
fn markdown_thematic_breaks_48() {
    // https://spec.commonmark.org/0.30/#example-48
    test_identical_markdown_events!(r##"    ***"##);
}

#[test]
fn markdown_thematic_breaks_49() {
    // https://spec.commonmark.org/0.30/#example-49
    test!(r##"Foo
    ***"##,r##"Foo
\***"##);
}

#[test]
fn markdown_thematic_breaks_50() {
    // https://spec.commonmark.org/0.30/#example-50
    test_identical_markdown_events!(r##"_____________________________________"##);
}

#[test]
fn markdown_thematic_breaks_51() {
    // https://spec.commonmark.org/0.30/#example-51
    test_identical_markdown_events!(r##" - - -"##,r##"- - -"##);
}

#[test]
fn markdown_thematic_breaks_52() {
    // https://spec.commonmark.org/0.30/#example-52
    test_identical_markdown_events!(r##" **  * ** * ** * **"##,r##"**  * ** * ** * **"##);
}

#[test]
fn markdown_thematic_breaks_53() {
    // https://spec.commonmark.org/0.30/#example-53
    test_identical_markdown_events!(r##"-     -      -      -"##);
}

#[test]
fn markdown_thematic_breaks_54() {
    // https://spec.commonmark.org/0.30/#example-54
    test_identical_markdown_events!("- - - -    ");
}

#[test]
fn markdown_thematic_breaks_55() {
    // https://spec.commonmark.org/0.30/#example-55
    test_identical_markdown_events!(r##"_ _ _ _ a

a------

---a---"##);
}

#[test]
fn markdown_thematic_breaks_56() {
    // https://spec.commonmark.org/0.30/#example-56
    test_identical_markdown_events!(r##" *-*"##,r##"*-*"##);
}

#[test]
fn markdown_thematic_breaks_57() {
    // https://spec.commonmark.org/0.30/#example-57
    test_identical_markdown_events!(r##"- foo
***
- bar"##);
}

#[test]
fn markdown_thematic_breaks_58() {
    // https://spec.commonmark.org/0.30/#example-58
    test_identical_markdown_events!(r##"Foo
***
bar"##);
}

#[test]
fn markdown_thematic_breaks_59() {
    // https://spec.commonmark.org/0.30/#example-59
    test_identical_markdown_events!(r##"Foo
---
bar"##);
}

#[test]
fn markdown_thematic_breaks_60() {
    // https://spec.commonmark.org/0.30/#example-60
    test_identical_markdown_events!(r##"* Foo
* * *
* Bar"##);
}

#[test]
fn markdown_thematic_breaks_61() {
    // https://spec.commonmark.org/0.30/#example-61
    test_identical_markdown_events!(r##"- Foo
- * * *"##);
}

#[test]
fn markdown_atx_headings_62() {
    // https://spec.commonmark.org/0.30/#example-62
    test_identical_markdown_events!(r##"# foo
## foo
### foo
#### foo
##### foo
###### foo"##);
}

#[test]
fn markdown_atx_headings_63() {
    // https://spec.commonmark.org/0.30/#example-63
    test_identical_markdown_events!(r##"####### foo"##);
}

#[test]
fn markdown_atx_headings_64() {
    // https://spec.commonmark.org/0.30/#example-64
    test_identical_markdown_events!(r##"#5 bolt

#hashtag"##);
}

#[test]
fn markdown_atx_headings_65() {
    // https://spec.commonmark.org/0.30/#example-65
    test_identical_markdown_events!(r##"\## foo"##);
}

#[test]
fn markdown_atx_headings_66() {
    // https://spec.commonmark.org/0.30/#example-66
    test_identical_markdown_events!(r##"# foo *bar* \*baz\*"##);
}

#[test]
fn markdown_atx_headings_67() {
    // https://spec.commonmark.org/0.30/#example-67
    test_identical_markdown_events!("#                  foo                     ",r##"# foo"##);
}

#[test]
fn markdown_atx_headings_68() {
    // https://spec.commonmark.org/0.30/#example-68
    test_identical_markdown_events!(r##" ### foo
  ## foo
   # foo"##,r##"### foo
## foo
# foo"##);
}

#[test]
fn markdown_atx_headings_69() {
    // https://spec.commonmark.org/0.30/#example-69
    test_identical_markdown_events!(r##"    # foo"##);
}

#[test]
fn markdown_atx_headings_70() {
    // https://spec.commonmark.org/0.30/#example-70
    test_identical_markdown_events!(r##"foo
    # bar"##,r##"foo
\# bar"##);
}

#[test]
fn markdown_atx_headings_71() {
    // https://spec.commonmark.org/0.30/#example-71
    test_identical_markdown_events!(r##"## foo ##
  ###   bar    ###"##,r##"## foo
### bar"##);
}

#[test]
fn markdown_atx_headings_72() {
    // https://spec.commonmark.org/0.30/#example-72
    test_identical_markdown_events!(r##"# foo ##################################
##### foo ##"##,r##"# foo
##### foo"##);
}

#[test]
fn markdown_atx_headings_73() {
    // https://spec.commonmark.org/0.30/#example-73
    test_identical_markdown_events!("### foo ###     ",r##"### foo"##);
}

#[test]
fn markdown_atx_headings_74() {
    // https://spec.commonmark.org/0.30/#example-74
    test_identical_markdown_events!(r##"### foo ### b"##);
}

#[test]
fn markdown_atx_headings_75() {
    // https://spec.commonmark.org/0.30/#example-75
    test_identical_markdown_events!(r##"# foo#"##);
}

#[test]
fn markdown_atx_headings_76() {
    // https://spec.commonmark.org/0.30/#example-76
    test_identical_markdown_events!(r##"### foo \###
## foo #\##
# foo \#"##);
}

#[test]
fn markdown_atx_headings_77() {
    // https://spec.commonmark.org/0.30/#example-77
    test_identical_markdown_events!(r##"****
## foo
****"##);
}

#[test]
fn markdown_atx_headings_78() {
    // https://spec.commonmark.org/0.30/#example-78
    test_identical_markdown_events!(r##"Foo bar
# baz
Bar foo"##);
}

#[test]
fn markdown_atx_headings_79() {
    // https://spec.commonmark.org/0.30/#example-79
    test_identical_markdown_events!("## \n#\n### ###",r##"##
#
###"##);
}

#[test]
fn markdown_setext_headings_80() {
    // https://spec.commonmark.org/0.30/#example-80
    test_identical_markdown_events!(r##"Foo *bar*
=========

Foo *bar*
---------"##);
}

#[test]
fn markdown_setext_headings_81() {
    // https://spec.commonmark.org/0.30/#example-81
    test_identical_markdown_events!(r##"Foo *bar
baz*
===="##);
}

#[test]
fn markdown_setext_headings_82() {
    // https://spec.commonmark.org/0.30/#example-82
    test_identical_markdown_events!("  Foo *bar\nbaz*\t\n====",r##"Foo *bar
baz*
===="##);
}

#[test]
fn markdown_setext_headings_83() {
    // https://spec.commonmark.org/0.30/#example-83
    test_identical_markdown_events!(r##"Foo
-------------------------

Foo
="##);
}

#[test]
fn markdown_setext_headings_84() {
    // https://spec.commonmark.org/0.30/#example-84
    test_identical_markdown_events!(r##"   Foo
---

  Foo
-----

  Foo
  ==="##,r##"Foo
---

Foo
-----

Foo
==="##);
}

#[test]
fn markdown_setext_headings_85() {
    // https://spec.commonmark.org/0.30/#example-85
    test_identical_markdown_events!(r##"    Foo
    ---

    Foo
---"##);
}

#[test]
fn markdown_setext_headings_86() {
    // https://spec.commonmark.org/0.30/#example-86
    test_identical_markdown_events!("Foo\n   ----      ",r##"Foo
----"##);
}

#[test]
fn markdown_setext_headings_87() {
    // https://spec.commonmark.org/0.30/#example-87
    test_identical_markdown_events!(r##"Foo
    ---"##,r##"Foo
\---"##);
}

#[test]
fn markdown_setext_headings_88() {
    // https://spec.commonmark.org/0.30/#example-88
    test_identical_markdown_events!(r##"Foo
= =

Foo
--- -"##);
}

#[test]
fn markdown_setext_headings_89() {
    // https://spec.commonmark.org/0.30/#example-89
    test_identical_markdown_events!("Foo  \n-----",r##"Foo
-----"##);
}

#[test]
fn markdown_setext_headings_90() {
    // https://spec.commonmark.org/0.30/#example-90
    test_identical_markdown_events!(r##"Foo\
----"##);
}

#[test]
fn markdown_setext_headings_91() {
    // https://spec.commonmark.org/0.30/#example-91
    test_identical_markdown_events!(r##"`Foo
----
`

<a title="a lot
---
of dashes"/>"##);
}

#[test]
fn markdown_setext_headings_92() {
    // https://spec.commonmark.org/0.30/#example-92
    test_identical_markdown_events!(r##"> Foo
---"##);
}

#[test]
fn markdown_setext_headings_93() {
    // https://spec.commonmark.org/0.30/#example-93
    test_identical_markdown_events!(r##"> foo
bar
==="##,r##"> foo
> bar
> \==="##);
}

#[test]
fn markdown_setext_headings_94() {
    // https://spec.commonmark.org/0.30/#example-94
    test_identical_markdown_events!(r##"- Foo
---"##);
}

#[test]
fn markdown_setext_headings_95() {
    // https://spec.commonmark.org/0.30/#example-95
    test_identical_markdown_events!(r##"Foo
Bar
---"##);
}

#[test]
fn markdown_setext_headings_96() {
    // https://spec.commonmark.org/0.30/#example-96
    test_identical_markdown_events!(r##"---
Foo
---
Bar
---
Baz"##);
}

#[test]
fn markdown_setext_headings_97() {
    // https://spec.commonmark.org/0.30/#example-97
    test_identical_markdown_events!(r##"
===="##,r##"===="##);
}

#[test]
fn markdown_setext_headings_98() {
    // https://spec.commonmark.org/0.30/#example-98
    test_identical_markdown_events!(r##"---
---"##);
}

#[test]
fn markdown_setext_headings_99() {
    // https://spec.commonmark.org/0.30/#example-99
    test_identical_markdown_events!(r##"- foo
-----"##);
}

#[test]
fn markdown_setext_headings_100() {
    // https://spec.commonmark.org/0.30/#example-100
    test_identical_markdown_events!(r##"    foo
---"##);
}

#[test]
fn markdown_setext_headings_101() {
    // https://spec.commonmark.org/0.30/#example-101
    test_identical_markdown_events!(r##"> foo
-----"##);
}

#[test]
fn markdown_setext_headings_102() {
    // https://spec.commonmark.org/0.30/#example-102
    test_identical_markdown_events!(r##"\> foo
------"##);
}

#[test]
fn markdown_setext_headings_103() {
    // https://spec.commonmark.org/0.30/#example-103
    test_identical_markdown_events!(r##"Foo

bar
---
baz"##);
}

#[test]
fn markdown_setext_headings_104() {
    // https://spec.commonmark.org/0.30/#example-104
    test_identical_markdown_events!(r##"Foo
bar

---

baz"##);
}

#[test]
fn markdown_setext_headings_105() {
    // https://spec.commonmark.org/0.30/#example-105
    test_identical_markdown_events!(r##"Foo
bar
* * *
baz"##);
}

#[test]
fn markdown_setext_headings_106() {
    // https://spec.commonmark.org/0.30/#example-106
    test_identical_markdown_events!(r##"Foo
bar
\---
baz"##);
}

#[test]
fn markdown_indented_code_blocks_107() {
    // https://spec.commonmark.org/0.30/#example-107
    test_identical_markdown_events!(r##"    a simple
      indented code block"##);
}

#[test]
fn markdown_indented_code_blocks_108() {
    // https://spec.commonmark.org/0.30/#example-108
    test_identical_markdown_events!(r##"  - foo

    bar"##,r##"- foo

  bar"##);
}

#[test]
fn markdown_indented_code_blocks_109() {
    // https://spec.commonmark.org/0.30/#example-109
    test_identical_markdown_events!(r##"1.  foo

    - bar"##,r##"1. foo

   - bar"##);
}

#[test]
fn markdown_indented_code_blocks_110() {
    // https://spec.commonmark.org/0.30/#example-110
    test_identical_markdown_events!(r##"    <a/>
    *hi*

    - one"##);
}

#[test]
fn markdown_indented_code_blocks_111() {
    // https://spec.commonmark.org/0.30/#example-111
    test_identical_markdown_events!("    chunk1\n\n    chunk2\n  \n \n \n    chunk3",r##"    chunk1

    chunk2



    chunk3"##);
}

#[test]
fn markdown_indented_code_blocks_112() {
    // https://spec.commonmark.org/0.30/#example-112
    test_identical_markdown_events!("    chunk1\n      \n      chunk2",r##"    chunk1

      chunk2"##);
}

#[test]
fn markdown_indented_code_blocks_113() {
    // https://spec.commonmark.org/0.30/#example-113
    test_identical_markdown_events!(r##"Foo
    bar"##,r##"Foo
bar"##);
}

#[test]
fn markdown_indented_code_blocks_114() {
    // https://spec.commonmark.org/0.30/#example-114
    test_identical_markdown_events!(r##"    foo
bar"##);
}

#[test]
fn markdown_indented_code_blocks_115() {
    // https://spec.commonmark.org/0.30/#example-115
    test_identical_markdown_events!(r##"# Heading
    foo
Heading
------
    foo
----"##);
}

#[test]
fn markdown_indented_code_blocks_116() {
    // https://spec.commonmark.org/0.30/#example-116
    test_identical_markdown_events!(r##"        foo
    bar"##);
}

#[test]
fn markdown_indented_code_blocks_117() {
    // https://spec.commonmark.org/0.30/#example-117
    test_identical_markdown_events!("\n    \n    foo\n    ",r##"    foo"##);
}

#[test]
fn markdown_indented_code_blocks_118() {
    // https://spec.commonmark.org/0.30/#example-118
    test_identical_markdown_events!("    foo  ",r##"    foo"##);
}

#[test]
fn markdown_fenced_code_blocks_119() {
    // https://spec.commonmark.org/0.30/#example-119
    test_identical_markdown_events!(r##"```
<
 >
```"##);
}

#[test]
fn markdown_fenced_code_blocks_120() {
    // https://spec.commonmark.org/0.30/#example-120
    test_identical_markdown_events!(r##"~~~
<
 >
~~~"##);
}

#[test]
fn markdown_fenced_code_blocks_121() {
    // https://spec.commonmark.org/0.30/#example-121
    test_identical_markdown_events!(r##"``
foo
``"##);
}

#[test]
fn markdown_fenced_code_blocks_122() {
    // https://spec.commonmark.org/0.30/#example-122
    test_identical_markdown_events!(r##"```
aaa
~~~
```"##);
}

#[test]
fn markdown_fenced_code_blocks_123() {
    // https://spec.commonmark.org/0.30/#example-123
    test_identical_markdown_events!(r##"~~~
aaa
```
~~~"##);
}

#[test]
fn markdown_fenced_code_blocks_124() {
    // https://spec.commonmark.org/0.30/#example-124
    test_identical_markdown_events!(r##"````
aaa
```
``````"##,r##"````
aaa
```
````"##);
}

#[test]
fn markdown_fenced_code_blocks_125() {
    // https://spec.commonmark.org/0.30/#example-125
    test_identical_markdown_events!(r##"~~~~
aaa
~~~
~~~~"##);
}

#[test]
fn markdown_fenced_code_blocks_126() {
    // https://spec.commonmark.org/0.30/#example-126
    test_identical_markdown_events!(r##"```"##,r##"```
```"##);
}

#[test]
fn markdown_fenced_code_blocks_127() {
    // https://spec.commonmark.org/0.30/#example-127
    test!(r##"`````

```
aaa"##,r##"`````

```
aaa
`````"##);
}

#[test]
fn markdown_fenced_code_blocks_128() {
    // https://spec.commonmark.org/0.30/#example-128
    test_identical_markdown_events!(r##"> ```
> aaa

bbb"##,r##"> ```
> aaa
> ```

bbb"##);
}

#[test]
fn markdown_fenced_code_blocks_129() {
    // https://spec.commonmark.org/0.30/#example-129
    test_identical_markdown_events!("```\n\n  \n```",r##"```
```"##);
}

#[test]
fn markdown_fenced_code_blocks_130() {
    // https://spec.commonmark.org/0.30/#example-130
    test_identical_markdown_events!(r##"```
```"##);
}

#[test]
fn markdown_fenced_code_blocks_131() {
    // https://spec.commonmark.org/0.30/#example-131
    test_identical_markdown_events!(r##" ```
 aaa
aaa
```"##,r##"```
aaa
aaa
```"##);
}

#[test]
fn markdown_fenced_code_blocks_132() {
    // https://spec.commonmark.org/0.30/#example-132
    test!(r##"  ```
aaa
  aaa
aaa
  ```"##,r##"```
aaa
aaa
aaa
```"##);
}

#[test]
fn markdown_fenced_code_blocks_133() {
    // https://spec.commonmark.org/0.30/#example-133
    test!(r##"   ```
   aaa
    aaa
  aaa
   ```"##,r##"```
aaa
 aaa
aaa
```"##);
}

#[test]
fn markdown_fenced_code_blocks_134() {
    // https://spec.commonmark.org/0.30/#example-134
    test_identical_markdown_events!(r##"    ```
    aaa
    ```"##);
}

#[test]
fn markdown_fenced_code_blocks_135() {
    // https://spec.commonmark.org/0.30/#example-135
    test_identical_markdown_events!(r##"```
aaa
  ```"##,r##"```
aaa
```"##);
}

#[test]
fn markdown_fenced_code_blocks_136() {
    // https://spec.commonmark.org/0.30/#example-136
    test_identical_markdown_events!(r##"   ```
aaa
  ```"##,r##"```
aaa
```"##);
}

#[test]
fn markdown_fenced_code_blocks_137() {
    // https://spec.commonmark.org/0.30/#example-137
    test!(r##"```
aaa
    ```"##,r##"```
aaa
    ```
```"##);
}

#[test]
fn markdown_fenced_code_blocks_138() {
    // https://spec.commonmark.org/0.30/#example-138
    test_identical_markdown_events!(r##"``` ```
aaa"##);
}

#[test]
fn markdown_fenced_code_blocks_139() {
    // https://spec.commonmark.org/0.30/#example-139
    test!(r##"~~~~~~
aaa
~~~ ~~"##,r##"~~~~~~
aaa
~~~ ~~
~~~~~~"##);
}

#[test]
fn markdown_fenced_code_blocks_140() {
    // https://spec.commonmark.org/0.30/#example-140
    test_identical_markdown_events!(r##"foo
```
bar
```
baz"##);
}

#[test]
fn markdown_fenced_code_blocks_141() {
    // https://spec.commonmark.org/0.30/#example-141
    test_identical_markdown_events!(r##"foo
---
~~~
bar
~~~
# baz"##);
}

#[test]
fn markdown_fenced_code_blocks_142() {
    // https://spec.commonmark.org/0.30/#example-142
    test_identical_markdown_events!(r##"```ruby
def foo(x)
  return 3
end
```"##);
}

#[test]
fn markdown_fenced_code_blocks_143() {
    // https://spec.commonmark.org/0.30/#example-143
    test_identical_markdown_events!(r##"~~~~    ruby startline=3 $%@#$
def foo(x)
  return 3
end
~~~~~~~"##,r##"~~~~ ruby startline=3 $%@#$
def foo(x)
  return 3
end
~~~~"##);
}

#[test]
fn markdown_fenced_code_blocks_144() {
    // https://spec.commonmark.org/0.30/#example-144
    test_identical_markdown_events!(r##"````;
````"##);
}

#[test]
fn markdown_fenced_code_blocks_145() {
    // https://spec.commonmark.org/0.30/#example-145
    test_identical_markdown_events!(r##"``` aa ```
foo"##);
}

#[test]
fn markdown_fenced_code_blocks_146() {
    // https://spec.commonmark.org/0.30/#example-146
    test_identical_markdown_events!(r##"~~~ aa ``` ~~~
foo
~~~"##);
}

#[test]
fn markdown_fenced_code_blocks_147() {
    // https://spec.commonmark.org/0.30/#example-147
    test_identical_markdown_events!(r##"```
``` aaa
```"##);
}

#[test]
fn markdown_html_blocks_148() {
    // https://spec.commonmark.org/0.30/#example-148
    test_identical_markdown_events!(r##"<table><tr><td>
<pre>
**Hello**,

_world_.
</pre>
</td></tr></table>"##);
}

#[test]
fn markdown_html_blocks_149() {
    // https://spec.commonmark.org/0.30/#example-149
    test_identical_markdown_events!(r##"<table>
  <tr>
    <td>
           hi
    </td>
  </tr>
</table>

okay."##);
}

#[test]
fn markdown_html_blocks_150() {
    // https://spec.commonmark.org/0.30/#example-150
    test_identical_markdown_events!(r##" <div>
  *hello*
         <foo><a>"##,r##"<div>
  *hello*
         <foo><a>"##);
}

#[test]
fn markdown_html_blocks_151() {
    // https://spec.commonmark.org/0.30/#example-151
    test_identical_markdown_events!(r##"</div>
*foo*"##);
}

#[test]
fn markdown_html_blocks_152() {
    // https://spec.commonmark.org/0.30/#example-152
    test_identical_markdown_events!(r##"<DIV CLASS="foo">

*Markdown*

</DIV>"##);
}

#[test]
fn markdown_html_blocks_153() {
    // https://spec.commonmark.org/0.30/#example-153
    test_identical_markdown_events!(r##"<div id="foo"
  class="bar">
</div>"##);
}

#[test]
fn markdown_html_blocks_154() {
    // https://spec.commonmark.org/0.30/#example-154
    test_identical_markdown_events!(r##"<div id="foo" class="bar
  baz">
</div>"##);
}

#[test]
fn markdown_html_blocks_155() {
    // https://spec.commonmark.org/0.30/#example-155
    test_identical_markdown_events!(r##"<div>
*foo*

*bar*"##);
}

#[test]
fn markdown_html_blocks_156() {
    // https://spec.commonmark.org/0.30/#example-156
    test_identical_markdown_events!(r##"<div id="foo"
*hi*"##);
}

#[test]
fn markdown_html_blocks_157() {
    // https://spec.commonmark.org/0.30/#example-157
    test_identical_markdown_events!(r##"<div class
foo"##);
}

#[test]
fn markdown_html_blocks_158() {
    // https://spec.commonmark.org/0.30/#example-158
    test_identical_markdown_events!(r##"<div *???-&&&-<---
*foo*"##);
}

#[test]
fn markdown_html_blocks_159() {
    // https://spec.commonmark.org/0.30/#example-159
    test_identical_markdown_events!(r##"<div><a href="bar">*foo*</a></div>"##);
}

#[test]
fn markdown_html_blocks_160() {
    // https://spec.commonmark.org/0.30/#example-160
    test_identical_markdown_events!(r##"<table><tr><td>
foo
</td></tr></table>"##);
}

#[test]
fn markdown_html_blocks_161() {
    // https://spec.commonmark.org/0.30/#example-161
    test_identical_markdown_events!(r##"<div></div>
``` c
int x = 33;
```"##);
}

#[test]
fn markdown_html_blocks_162() {
    // https://spec.commonmark.org/0.30/#example-162
    test_identical_markdown_events!(r##"<a href="foo">
*bar*
</a>"##);
}

#[test]
fn markdown_html_blocks_163() {
    // https://spec.commonmark.org/0.30/#example-163
    test_identical_markdown_events!(r##"<Warning>
*bar*
</Warning>"##);
}

#[test]
fn markdown_html_blocks_164() {
    // https://spec.commonmark.org/0.30/#example-164
    test_identical_markdown_events!(r##"<i class="foo">
*bar*
</i>"##);
}

#[test]
fn markdown_html_blocks_165() {
    // https://spec.commonmark.org/0.30/#example-165
    test_identical_markdown_events!(r##"</ins>
*bar*"##);
}

#[test]
fn markdown_html_blocks_166() {
    // https://spec.commonmark.org/0.30/#example-166
    test_identical_markdown_events!(r##"<del>
*foo*
</del>"##);
}

#[test]
fn markdown_html_blocks_167() {
    // https://spec.commonmark.org/0.30/#example-167
    test_identical_markdown_events!(r##"<del>

*foo*

</del>"##);
}

#[test]
fn markdown_html_blocks_168() {
    // https://spec.commonmark.org/0.30/#example-168
    test_identical_markdown_events!(r##"<del>*foo*</del>"##);
}

#[test]
fn markdown_html_blocks_169() {
    // https://spec.commonmark.org/0.30/#example-169
    test_identical_markdown_events!(r##"<pre language="haskell"><code>
import Text.HTML.TagSoup

main :: IO ()
main = print $ parseTags tags
</code></pre>
okay"##);
}

#[test]
fn markdown_html_blocks_170() {
    // https://spec.commonmark.org/0.30/#example-170
    test_identical_markdown_events!(r##"<script type="text/javascript">
// JavaScript example

document.getElementById("demo").innerHTML = "Hello JavaScript!";
</script>
okay"##);
}

#[test]
fn markdown_html_blocks_171() {
    // https://spec.commonmark.org/0.30/#example-171
    test_identical_markdown_events!(r##"<textarea>

*foo*

_bar_

</textarea>"##);
}

#[test]
fn markdown_html_blocks_172() {
    // https://spec.commonmark.org/0.30/#example-172
    test_identical_markdown_events!(r##"<style
  type="text/css">
h1 {color:red;}

p {color:blue;}
</style>
okay"##);
}

#[test]
fn markdown_html_blocks_173() {
    // https://spec.commonmark.org/0.30/#example-173
    test_identical_markdown_events!(r##"<style
  type="text/css">

foo"##);
}

#[test]
fn markdown_html_blocks_174() {
    // https://spec.commonmark.org/0.30/#example-174
    test_identical_markdown_events!(r##"> <div>
> foo

bar"##);
}

#[test]
fn markdown_html_blocks_175() {
    // https://spec.commonmark.org/0.30/#example-175
    test_identical_markdown_events!(r##"- <div>
- foo"##);
}

#[test]
fn markdown_html_blocks_176() {
    // https://spec.commonmark.org/0.30/#example-176
    test_identical_markdown_events!(r##"<style>p{color:red;}</style>
*foo*"##);
}

#[test]
fn markdown_html_blocks_177() {
    // https://spec.commonmark.org/0.30/#example-177
    test_identical_markdown_events!(r##"<!-- foo -->*bar*
*baz*"##);
}

#[test]
fn markdown_html_blocks_178() {
    // https://spec.commonmark.org/0.30/#example-178
    test_identical_markdown_events!(r##"<script>
foo
</script>1. *bar*"##);
}

#[test]
fn markdown_html_blocks_179() {
    // https://spec.commonmark.org/0.30/#example-179
    test_identical_markdown_events!(r##"<!-- Foo

bar
   baz -->
okay"##);
}

#[test]
fn markdown_html_blocks_180() {
    // https://spec.commonmark.org/0.30/#example-180
    test_identical_markdown_events!(r##"<?php

  echo '>';

?>
okay"##);
}

#[test]
fn markdown_html_blocks_181() {
    // https://spec.commonmark.org/0.30/#example-181
    test_identical_markdown_events!(r##"<!DOCTYPE html>"##);
}

#[test]
fn markdown_html_blocks_182() {
    // https://spec.commonmark.org/0.30/#example-182
    test_identical_markdown_events!(r##"<![CDATA[
function matchwo(a,b)
{
  if (a < b && a < 0) then {
    return 1;

  } else {

    return 0;
  }
}
]]>
okay"##);
}

#[test]
fn markdown_html_blocks_183() {
    // https://spec.commonmark.org/0.30/#example-183
    test_identical_markdown_events!(r##"  <!-- foo -->

    <!-- foo -->"##,r##"<!-- foo -->

    <!-- foo -->"##);
}

#[test]
fn markdown_html_blocks_184() {
    // https://spec.commonmark.org/0.30/#example-184
    test_identical_markdown_events!(r##"  <div>

    <div>"##,r##"<div>

    <div>"##);
}

#[test]
fn markdown_html_blocks_185() {
    // https://spec.commonmark.org/0.30/#example-185
    test_identical_markdown_events!(r##"Foo
<div>
bar
</div>"##);
}

#[test]
fn markdown_html_blocks_186() {
    // https://spec.commonmark.org/0.30/#example-186
    test_identical_markdown_events!(r##"<div>
bar
</div>
*foo*"##);
}

#[test]
fn markdown_html_blocks_187() {
    // https://spec.commonmark.org/0.30/#example-187
    test_identical_markdown_events!(r##"Foo
<a href="bar">
baz"##);
}

#[test]
fn markdown_html_blocks_188() {
    // https://spec.commonmark.org/0.30/#example-188
    test_identical_markdown_events!(r##"<div>

*Emphasized* text.

</div>"##);
}

#[test]
fn markdown_html_blocks_189() {
    // https://spec.commonmark.org/0.30/#example-189
    test_identical_markdown_events!(r##"<div>
*Emphasized* text.
</div>"##);
}

#[test]
fn markdown_html_blocks_190() {
    // https://spec.commonmark.org/0.30/#example-190
    test_identical_markdown_events!(r##"<table>

<tr>

<td>
Hi
</td>

</tr>

</table>"##);
}

#[test]
fn markdown_html_blocks_191() {
    // https://spec.commonmark.org/0.30/#example-191
    test_identical_markdown_events!(r##"<table>

  <tr>

    <td>
      Hi
    </td>

  </tr>

</table>"##,r##"<table>

<tr>

    <td>
      Hi
    </td>

</tr>

</table>"##);
}

#[test]
fn markdown_link_reference_definitions_192() {
    // https://spec.commonmark.org/0.30/#example-192
    test_identical_markdown_events!(r##"[foo]: /url "title"

[foo]"##,r##"[foo]: /url "title"

[foo]"##);
}

#[test]
fn markdown_link_reference_definitions_193() {
    // https://spec.commonmark.org/0.30/#example-193
    test_identical_markdown_events!("   [foo]: \n      /url  \n           'the title'  \n\n[foo]",r##"[foo]: /url 'the title'

[foo]"##);
}

#[test]
fn markdown_link_reference_definitions_194() {
    // https://spec.commonmark.org/0.30/#example-194
    test_identical_markdown_events!(r##"[Foo*bar\]]:my_(url) 'title (with parens)'

[Foo*bar\]]"##,r##"[Foo*bar\]]: my_(url) 'title (with parens)'

[Foo*bar\]]"##);
}

#[test]
fn markdown_link_reference_definitions_195() {
    // https://spec.commonmark.org/0.30/#example-195
    test_identical_markdown_events!(r##"[Foo bar]:
<my url>
'title'

[Foo bar]"##,r##"[Foo bar]: <my url> 'title'

[Foo bar]"##);
}

#[test]
fn markdown_link_reference_definitions_196() {
    // https://spec.commonmark.org/0.30/#example-196
    test_identical_markdown_events!(r##"[foo]: /url '
title
line1
line2
'

[foo]"##,r##"[foo]: /url '
title
line1
line2
'

[foo]"##);
}

#[test]
fn markdown_link_reference_definitions_197() {
    // https://spec.commonmark.org/0.30/#example-197
    test_identical_markdown_events!(r##"[foo]: /url 'title

with blank line'

[foo]"##);
}

#[test]
fn markdown_link_reference_definitions_198() {
    // https://spec.commonmark.org/0.30/#example-198
    test_identical_markdown_events!(r##"[foo]:
/url

[foo]"##,r##"[foo]: /url

[foo]"##);
}

#[test]
fn markdown_link_reference_definitions_199() {
    // https://spec.commonmark.org/0.30/#example-199
    test_identical_markdown_events!(r##"[foo]:

[foo]"##);
}

#[test]
fn markdown_link_reference_definitions_200() {
    // https://spec.commonmark.org/0.30/#example-200
    test_identical_markdown_events!(r##"[foo]: <>

[foo]"##,r##"[foo]: <>

[foo]"##);
}

#[test]
fn markdown_link_reference_definitions_201() {
    // https://spec.commonmark.org/0.30/#example-201
    test_identical_markdown_events!(r##"[foo]: <bar>(baz)

[foo]"##);
}

#[test]
fn markdown_link_reference_definitions_202() {
    // https://spec.commonmark.org/0.30/#example-202
    test_identical_markdown_events!(r##"[foo]: /url\bar\*baz "foo\"bar\baz"

[foo]"##,r##"[foo]: /url\bar\*baz "foo\"bar\baz"

[foo]"##);
}

#[test]
fn markdown_link_reference_definitions_203() {
    // https://spec.commonmark.org/0.30/#example-203
    test_identical_markdown_events!(r##"[foo]

[foo]: url"##);
}

#[test]
fn markdown_link_reference_definitions_204() {
    // https://spec.commonmark.org/0.30/#example-204
    test_identical_markdown_events!(r##"[foo]

[foo]: first
[foo]: second"##,r##"[foo]

[foo]: first"##);
}

#[test]
fn markdown_link_reference_definitions_205() {
    // https://spec.commonmark.org/0.30/#example-205
    test_identical_markdown_events!(r##"[FOO]: /url

[Foo]"##);
}

#[test]
fn markdown_link_reference_definitions_206() {
    // https://spec.commonmark.org/0.30/#example-206
    test_identical_markdown_events!(r##"[ΑΓΩ]: /φου

[αγω]"##,r##"[ΑΓΩ]: /φου

[αγω]"##);
}

#[test]
fn markdown_link_reference_definitions_207() {
    // https://spec.commonmark.org/0.30/#example-207
    test_identical_markdown_events!(r##"[foo]: /url"##);
}

#[test]
fn markdown_link_reference_definitions_208() {
    // https://spec.commonmark.org/0.30/#example-208
    test_identical_markdown_events!(r##"[
foo
]: /url
bar"##,r##"[ foo ]: /url
bar"##);
}

#[test]
fn markdown_link_reference_definitions_209() {
    // https://spec.commonmark.org/0.30/#example-209
    test_identical_markdown_events!(r##"[foo]: /url "title" ok"##);
}

// FIXME(ytmim) the "title" is duplcated here
#[ignore]
#[test]
fn markdown_link_reference_definitions_210() {
    // https://spec.commonmark.org/0.30/#example-210
    test_identical_markdown_events!(r##"[foo]: /url
"title" ok"##,r##"[foo]: /url "title""title" ok"##);
}

#[test]
fn markdown_link_reference_definitions_211() {
    // https://spec.commonmark.org/0.30/#example-211
    test_identical_markdown_events!(r##"    [foo]: /url "title"

[foo]"##);
}

#[test]
fn markdown_link_reference_definitions_212() {
    // https://spec.commonmark.org/0.30/#example-212
    test_identical_markdown_events!(r##"```
[foo]: /url
```

[foo]"##);
}

#[test]
fn markdown_link_reference_definitions_213() {
    // https://spec.commonmark.org/0.30/#example-213
    test_identical_markdown_events!(r##"Foo
[bar]: /baz

[bar]"##);
}

#[test]
fn markdown_link_reference_definitions_214() {
    // https://spec.commonmark.org/0.30/#example-214
    test_identical_markdown_events!(r##"# [Foo]
[foo]: /url
> bar"##);
}

#[test]
fn markdown_link_reference_definitions_215() {
    // https://spec.commonmark.org/0.30/#example-215
    test_identical_markdown_events!(r##"[foo]: /url
bar
===
[foo]"##);
}

#[test]
fn markdown_link_reference_definitions_216() {
    // https://spec.commonmark.org/0.30/#example-216
    test_identical_markdown_events!(r##"[foo]: /url
===
[foo]"##);
}

#[test]
fn markdown_link_reference_definitions_217() {
    // https://spec.commonmark.org/0.30/#example-217
    test_identical_markdown_events!(r##"[foo]: /foo-url "foo"
[bar]: /bar-url
  "bar"
[baz]: /baz-url

[foo],
[bar],
[baz]"##,r##"[foo]: /foo-url "foo"
[bar]: /bar-url "bar"
[baz]: /baz-url

[foo],
[bar],
[baz]"##);
}

#[test]
fn markdown_link_reference_definitions_218() {
    // https://spec.commonmark.org/0.30/#example-218
    test!(r##"[foo]

> [foo]: /url"##,r##"[foo]

>
[foo]: /url"##);
}

#[test]
fn markdown_paragraphs_219() {
    // https://spec.commonmark.org/0.30/#example-219
    test_identical_markdown_events!(r##"aaa

bbb"##);
}

#[test]
fn markdown_paragraphs_220() {
    // https://spec.commonmark.org/0.30/#example-220
    test_identical_markdown_events!(r##"aaa
bbb

ccc
ddd"##);
}

#[test]
fn markdown_paragraphs_221() {
    // https://spec.commonmark.org/0.30/#example-221
    test_identical_markdown_events!(r##"aaa


bbb"##);
}

#[test]
fn markdown_paragraphs_222() {
    // https://spec.commonmark.org/0.30/#example-222
    test_identical_markdown_events!(r##"  aaa
 bbb"##,r##"aaa
bbb"##);
}

#[test]
fn markdown_paragraphs_223() {
    // https://spec.commonmark.org/0.30/#example-223
    test_identical_markdown_events!(r##"aaa
             bbb
                                       ccc"##,r##"aaa
bbb
ccc"##);
}

#[test]
fn markdown_paragraphs_224() {
    // https://spec.commonmark.org/0.30/#example-224
    test_identical_markdown_events!(r##"   aaa
bbb"##,r##"aaa
bbb"##);
}

#[test]
fn markdown_paragraphs_225() {
    // https://spec.commonmark.org/0.30/#example-225
    test_identical_markdown_events!(r##"    aaa
bbb"##);
}

#[test]
fn markdown_paragraphs_226() {
    // https://spec.commonmark.org/0.30/#example-226
    test_identical_markdown_events!("aaa     \nbbb     ");
}

#[test]
fn markdown_blank_lines_227() {
    // https://spec.commonmark.org/0.30/#example-227
    test_identical_markdown_events!("  \n\naaa\n  \n\n# aaa\n\n  ",r##"aaa


# aaa"##);
}

#[test]
fn markdown_block_quotes_228() {
    // https://spec.commonmark.org/0.30/#example-228
    test_identical_markdown_events!(r##"> # Foo
> bar
> baz"##);
}

#[test]
fn markdown_block_quotes_229() {
    // https://spec.commonmark.org/0.30/#example-229
    test_identical_markdown_events!(r##"># Foo
>bar
> baz"##,r##"> # Foo
> bar
> baz"##);
}

#[test]
fn markdown_block_quotes_230() {
    // https://spec.commonmark.org/0.30/#example-230
    test_identical_markdown_events!(r##"   > # Foo
   > bar
 > baz"##,r##"> # Foo
> bar
> baz"##);
}

#[test]
fn markdown_block_quotes_231() {
    // https://spec.commonmark.org/0.30/#example-231
    test_identical_markdown_events!(r##"    > # Foo
    > bar
    > baz"##);
}

#[test]
fn markdown_block_quotes_232() {
    // https://spec.commonmark.org/0.30/#example-232
    test_identical_markdown_events!(r##"> # Foo
> bar
baz"##,r##"> # Foo
> bar
> baz"##);
}

#[test]
fn markdown_block_quotes_233() {
    // https://spec.commonmark.org/0.30/#example-233
    test_identical_markdown_events!(r##"> bar
baz
> foo"##,r##"> bar
> baz
> foo"##);
}

#[test]
fn markdown_block_quotes_234() {
    // https://spec.commonmark.org/0.30/#example-234
    test_identical_markdown_events!(r##"> foo
---"##);
}

#[test]
fn markdown_block_quotes_235() {
    // https://spec.commonmark.org/0.30/#example-235
    test_identical_markdown_events!(r##"> - foo
- bar"##);
}

#[test]
fn markdown_block_quotes_236() {
    // https://spec.commonmark.org/0.30/#example-236
    test_identical_markdown_events!(r##">     foo
    bar"##);
}

#[test]
fn markdown_block_quotes_237() {
    // https://spec.commonmark.org/0.30/#example-237
    test_identical_markdown_events!(r##"> ```
foo
```"##,r##"> ```
>
> ```
foo
```
```"##);
}

#[test]
fn markdown_block_quotes_238() {
    // https://spec.commonmark.org/0.30/#example-238
    test_identical_markdown_events!(r##"> foo
    - bar"##,r##"> foo
> \- bar"##);
}

#[test]
fn markdown_block_quotes_239() {
    // https://spec.commonmark.org/0.30/#example-239
    test_identical_markdown_events!(r##">"##);
}

#[test]
fn markdown_block_quotes_240() {
    // https://spec.commonmark.org/0.30/#example-240
    test_identical_markdown_events!(">\n>  \n> ",r##">
>
>"##);
}

#[test]
fn markdown_block_quotes_241() {
    // https://spec.commonmark.org/0.30/#example-241
    test_identical_markdown_events!(">\n> foo\n>  ",r##"> foo
>"##);
}

#[test]
fn markdown_block_quotes_242() {
    // https://spec.commonmark.org/0.30/#example-242
    test_identical_markdown_events!(r##"> foo

> bar"##);
}

#[test]
fn markdown_block_quotes_243() {
    // https://spec.commonmark.org/0.30/#example-243
    test_identical_markdown_events!(r##"> foo
> bar"##);
}

#[test]
fn markdown_block_quotes_244() {
    // https://spec.commonmark.org/0.30/#example-244
    test_identical_markdown_events!(r##"> foo
>
> bar"##);
}

#[test]
fn markdown_block_quotes_245() {
    // https://spec.commonmark.org/0.30/#example-245
    test_identical_markdown_events!(r##"foo
> bar"##);
}

#[test]
fn markdown_block_quotes_246() {
    // https://spec.commonmark.org/0.30/#example-246
    test_identical_markdown_events!(r##"> aaa
***
> bbb"##);
}

#[test]
fn markdown_block_quotes_247() {
    // https://spec.commonmark.org/0.30/#example-247
    test_identical_markdown_events!(r##"> bar
baz"##,r##"> bar
> baz"##);
}

#[test]
fn markdown_block_quotes_248() {
    // https://spec.commonmark.org/0.30/#example-248
    test_identical_markdown_events!(r##"> bar

baz"##);
}

#[test]
fn markdown_block_quotes_249() {
    // https://spec.commonmark.org/0.30/#example-249
    test_identical_markdown_events!(r##"> bar
>
baz"##);
}

#[test]
fn markdown_block_quotes_250() {
    // https://spec.commonmark.org/0.30/#example-250
    test_identical_markdown_events!(r##"> > > foo
bar"##,r##"> > > foo
> > > bar"##);
}

#[test]
fn markdown_block_quotes_251() {
    // https://spec.commonmark.org/0.30/#example-251
    test_identical_markdown_events!(r##">>> foo
> bar
>>baz"##,r##"> > > foo
> > > bar
> > > baz"##);
}

#[test]
fn markdown_block_quotes_252() {
    // https://spec.commonmark.org/0.30/#example-252
    test_identical_markdown_events!(r##">     code

>    not code"##,r##">     code

> not code"##);
}

#[test]
fn markdown_list_items_253() {
    // https://spec.commonmark.org/0.30/#example-253
    test_identical_markdown_events!(r##"A paragraph
with two lines.

    indented code

> A block quote."##);
}

#[test]
fn markdown_list_items_254() {
    // https://spec.commonmark.org/0.30/#example-254
    test_identical_markdown_events!(r##"1.  A paragraph
    with two lines.

        indented code

    > A block quote."##,r##"1. A paragraph
   with two lines.

       indented code

   > A block quote."##);
}

#[test]
fn markdown_list_items_255() {
    // https://spec.commonmark.org/0.30/#example-255
    test_identical_markdown_events!(r##"- one

 two"##,r##"- one

two"##);
}

#[test]
fn markdown_list_items_256() {
    // https://spec.commonmark.org/0.30/#example-256
    test_identical_markdown_events!(r##"- one

  two"##);
}

#[test]
fn markdown_list_items_257() {
    // https://spec.commonmark.org/0.30/#example-257
    test!(r##" -    one

     two"##,r##"- one
<!-- Dont absorb code block into list -->
<!-- Consider a feenced code block instead -->

     two"##);
}

#[test]
fn markdown_list_items_258() {
    // https://spec.commonmark.org/0.30/#example-258
    test_identical_markdown_events!(r##" -    one

      two"##,r##"- one

  two"##);
}

#[test]
fn markdown_list_items_259() {
    // https://spec.commonmark.org/0.30/#example-259
    test_identical_markdown_events!(r##"   > > 1.  one
>>
>>     two"##,r##"> > 1. one
> >
> >    two"##);
}

#[test]
fn markdown_list_items_260() {
    // https://spec.commonmark.org/0.30/#example-260
    test_identical_markdown_events!(r##">>- one
>>
  >  > two"##,r##"> > - one
> >
> > two"##);
}

#[test]
fn markdown_list_items_261() {
    // https://spec.commonmark.org/0.30/#example-261
    test_identical_markdown_events!(r##"-one

2.two"##);
}

#[test]
fn markdown_list_items_262() {
    // https://spec.commonmark.org/0.30/#example-262
    test_identical_markdown_events!(r##"- foo


  bar"##,r##"- foo


  bar"##);
}

#[test]
fn markdown_list_items_263() {
    // https://spec.commonmark.org/0.30/#example-263
    test_identical_markdown_events!(r##"1.  foo

    ```
    bar
    ```

    baz

    > bam"##,r##"1. foo

   ```
   bar
   ```

   baz

   > bam"##);
}

#[test]
fn markdown_list_items_264() {
    // https://spec.commonmark.org/0.30/#example-264
    test_identical_markdown_events!(r##"- Foo

      bar


      baz"##);
}

#[test]
fn markdown_list_items_265() {
    // https://spec.commonmark.org/0.30/#example-265
    test_identical_markdown_events!(r##"123456789. ok"##);
}

#[test]
fn markdown_list_items_266() {
    // https://spec.commonmark.org/0.30/#example-266
    test_identical_markdown_events!(r##"1234567890. not ok"##);
}

#[test]
fn markdown_list_items_267() {
    // https://spec.commonmark.org/0.30/#example-267
    test_identical_markdown_events!(r##"0. ok"##);
}

#[test]
fn markdown_list_items_268() {
    // https://spec.commonmark.org/0.30/#example-268
    test_identical_markdown_events!(r##"003. ok"##);
}

#[test]
fn markdown_list_items_269() {
    // https://spec.commonmark.org/0.30/#example-269
    test_identical_markdown_events!(r##"-1. not ok"##);
}

#[test]
fn markdown_list_items_270() {
    // https://spec.commonmark.org/0.30/#example-270
    test_identical_markdown_events!(r##"- foo

      bar"##);
}

#[test]
fn markdown_list_items_271() {
    // https://spec.commonmark.org/0.30/#example-271
    test_identical_markdown_events!(r##"  10.  foo

           bar"##,r##"10. foo

        bar"##);
}

#[test]
fn markdown_list_items_272() {
    // https://spec.commonmark.org/0.30/#example-272
    test_identical_markdown_events!(r##"    indented code

paragraph

    more code"##);
}

#[test]
fn markdown_list_items_273() {
    // https://spec.commonmark.org/0.30/#example-273
    test_identical_markdown_events!(r##"1.     indented code

   paragraph

       more code"##);
}

#[test]
fn markdown_list_items_274() {
    // https://spec.commonmark.org/0.30/#example-274
    test_identical_markdown_events!(r##"1.      indented code

   paragraph

       more code"##);
}

#[test]
fn markdown_list_items_275() {
    // https://spec.commonmark.org/0.30/#example-275
    test_identical_markdown_events!(r##"   foo

bar"##,r##"foo

bar"##);
}

#[test]
fn markdown_list_items_276() {
    // https://spec.commonmark.org/0.30/#example-276
    test_identical_markdown_events!(r##"-    foo

  bar"##,r##"- foo

bar"##);
}

#[test]
fn markdown_list_items_277() {
    // https://spec.commonmark.org/0.30/#example-277
    test_identical_markdown_events!(r##"-  foo

   bar"##,r##"- foo

  bar"##);
}

#[test]
fn markdown_list_items_278() {
    // https://spec.commonmark.org/0.30/#example-278
    test_identical_markdown_events!(r##"-
  foo
-
  ```
  bar
  ```
-
      baz"##,r##"- foo
- ```
  bar
  ```
-     baz"##);
}

#[test]
fn markdown_list_items_279() {
    // https://spec.commonmark.org/0.30/#example-279
    test_identical_markdown_events!("-   \n  foo",r##"- foo"##);
}

#[test]
fn markdown_list_items_280() {
    // https://spec.commonmark.org/0.30/#example-280
    test_identical_markdown_events!(r##"-

  foo"##,r##"-

foo"##);
}

#[test]
fn markdown_list_items_281() {
    // https://spec.commonmark.org/0.30/#example-281
    test_identical_markdown_events!(r##"- foo
-
- bar"##);
}

#[test]
fn markdown_list_items_282() {
    // https://spec.commonmark.org/0.30/#example-282
    test_identical_markdown_events!("- foo\n-   \n- bar",r##"- foo
-
- bar"##);
}

#[test]
fn markdown_list_items_283() {
    // https://spec.commonmark.org/0.30/#example-283
    test_identical_markdown_events!(r##"1. foo
2.
3. bar"##);
}

#[test]
fn markdown_list_items_284() {
    // https://spec.commonmark.org/0.30/#example-284
    test_identical_markdown_events!(r##"*"##);
}

#[test]
fn markdown_list_items_285() {
    // https://spec.commonmark.org/0.30/#example-285
    test_identical_markdown_events!(r##"foo
*

foo
1."##);
}

#[test]
fn markdown_list_items_286() {
    // https://spec.commonmark.org/0.30/#example-286
    test_identical_markdown_events!(r##" 1.  A paragraph
     with two lines.

         indented code

     > A block quote."##,r##"1. A paragraph
   with two lines.

       indented code

   > A block quote."##);
}

#[test]
fn markdown_list_items_287() {
    // https://spec.commonmark.org/0.30/#example-287
    test_identical_markdown_events!(r##"  1.  A paragraph
      with two lines.

          indented code

      > A block quote."##,r##"1. A paragraph
   with two lines.

       indented code

   > A block quote."##);
}

#[test]
fn markdown_list_items_288() {
    // https://spec.commonmark.org/0.30/#example-288
    test_identical_markdown_events!(r##"   1.  A paragraph
       with two lines.

           indented code

       > A block quote."##,r##"1. A paragraph
   with two lines.

       indented code

   > A block quote."##);
}

#[test]
fn markdown_list_items_289() {
    // https://spec.commonmark.org/0.30/#example-289
    test_identical_markdown_events!(r##"    1.  A paragraph
        with two lines.

            indented code

        > A block quote."##);
}

#[test]
fn markdown_list_items_290() {
    // https://spec.commonmark.org/0.30/#example-290
    test_identical_markdown_events!(r##"  1.  A paragraph
with two lines.

          indented code

      > A block quote."##,r##"1. A paragraph
   with two lines.

       indented code

   > A block quote."##);
}

#[test]
fn markdown_list_items_291() {
    // https://spec.commonmark.org/0.30/#example-291
    test_identical_markdown_events!(r##"  1.  A paragraph
    with two lines."##,r##"1. A paragraph
   with two lines."##);
}

#[test]
fn markdown_list_items_292() {
    // https://spec.commonmark.org/0.30/#example-292
    test_identical_markdown_events!(r##"> 1. > Blockquote
continued here."##,r##"> 1. > Blockquote
>    > continued here."##);
}

#[test]
fn markdown_list_items_293() {
    // https://spec.commonmark.org/0.30/#example-293
    test_identical_markdown_events!(r##"> 1. > Blockquote
> continued here."##,r##"> 1. > Blockquote
>    > continued here."##);
}

#[test]
fn markdown_list_items_294() {
    // https://spec.commonmark.org/0.30/#example-294
    test_identical_markdown_events!(r##"- foo
  - bar
    - baz
      - boo"##);
}

#[test]
fn markdown_list_items_295() {
    // https://spec.commonmark.org/0.30/#example-295
    test_identical_markdown_events!(r##"- foo
 - bar
  - baz
   - boo"##,r##"- foo
- bar
- baz
- boo"##);
}

#[test]
fn markdown_list_items_296() {
    // https://spec.commonmark.org/0.30/#example-296
    test_identical_markdown_events!(r##"10) foo
    - bar"##);
}

#[test]
fn markdown_list_items_297() {
    // https://spec.commonmark.org/0.30/#example-297
    test_identical_markdown_events!(r##"10) foo
   - bar"##,r##"10) foo
- bar"##);
}

#[test]
fn markdown_list_items_298() {
    // https://spec.commonmark.org/0.30/#example-298
    test_identical_markdown_events!(r##"- - foo"##);
}

#[test]
fn markdown_list_items_299() {
    // https://spec.commonmark.org/0.30/#example-299
    test_identical_markdown_events!(r##"1. - 2. foo"##);
}

#[test]
fn markdown_list_items_300() {
    // https://spec.commonmark.org/0.30/#example-300
    test_identical_markdown_events!(r##"- # Foo
- Bar
  ---
  baz"##);
}

#[test]
fn markdown_lists_301() {
    // https://spec.commonmark.org/0.30/#example-301
    test_identical_markdown_events!(r##"- foo
- bar
+ baz"##);
}

#[test]
fn markdown_lists_302() {
    // https://spec.commonmark.org/0.30/#example-302
    test_identical_markdown_events!(r##"1. foo
2. bar
3) baz"##);
}

#[test]
fn markdown_lists_303() {
    // https://spec.commonmark.org/0.30/#example-303
    test_identical_markdown_events!(r##"Foo
- bar
- baz"##);
}

#[test]
fn markdown_lists_304() {
    // https://spec.commonmark.org/0.30/#example-304
    test_identical_markdown_events!(r##"The number of windows in my house is
14.  The number of doors is 6."##);
}

#[test]
fn markdown_lists_305() {
    // https://spec.commonmark.org/0.30/#example-305
    test_identical_markdown_events!(r##"The number of windows in my house is
1.  The number of doors is 6."##,r##"The number of windows in my house is
1. The number of doors is 6."##);
}

#[test]
fn markdown_lists_306() {
    // https://spec.commonmark.org/0.30/#example-306
    test_identical_markdown_events!(r##"- foo

- bar


- baz"##);
}

#[test]
fn markdown_lists_307() {
    // https://spec.commonmark.org/0.30/#example-307
    test_identical_markdown_events!(r##"- foo
  - bar
    - baz


      bim"##);
}

#[test]
fn markdown_lists_308() {
    // https://spec.commonmark.org/0.30/#example-308
    test_identical_markdown_events!(r##"- foo
- bar

<!-- -->

- baz
- bim"##);
}

#[test]
fn markdown_lists_309() {
    // https://spec.commonmark.org/0.30/#example-309
    test_identical_markdown_events!(r##"-   foo

    notcode

-   foo

<!-- -->

    code"##,r##"- foo

  notcode

- foo

<!-- -->

    code"##);
}

#[test]
fn markdown_lists_310() {
    // https://spec.commonmark.org/0.30/#example-310
    test_identical_markdown_events!(r##"- a
 - b
  - c
   - d
  - e
 - f
- g"##,r##"- a
- b
- c
- d
- e
- f
- g"##);
}

#[test]
fn markdown_lists_311() {
    // https://spec.commonmark.org/0.30/#example-311
    test_identical_markdown_events!(r##"1. a

  2. b

   3. c"##,r##"1. a

2. b

3. c"##);
}

#[test]
fn markdown_lists_312() {
    // https://spec.commonmark.org/0.30/#example-312
    test_identical_markdown_events!(r##"- a
 - b
  - c
   - d
    - e"##,r##"- a
- b
- c
- d
  \- e"##);
}

#[test]
fn markdown_lists_313() {
    // https://spec.commonmark.org/0.30/#example-313
    test!(r##"1. a

  2. b

    3. c"##,r##"1. a

2. b
<!-- Dont absorb code block into list -->
<!-- Consider a feenced code block instead -->

    3. c"##);
}

#[test]
fn markdown_lists_314() {
    // https://spec.commonmark.org/0.30/#example-314
    test_identical_markdown_events!(r##"- a
- b

- c"##);
}

#[test]
fn markdown_lists_315() {
    // https://spec.commonmark.org/0.30/#example-315
    test_identical_markdown_events!(r##"* a
*

* c"##);
}

#[test]
fn markdown_lists_316() {
    // https://spec.commonmark.org/0.30/#example-316
    test_identical_markdown_events!(r##"- a
- b

  c
- d"##);
}

#[test]
fn markdown_lists_317() {
    // https://spec.commonmark.org/0.30/#example-317
    test!(r##"- a
- b

  [ref]: /url
- d"##,r##"- a
- b

[ref]: /url
- d"##);
}

#[test]
fn markdown_lists_318() {
    // https://spec.commonmark.org/0.30/#example-318
    test!(r##"- a
- ```
  b


  ```
- c"##,r##"- a
- ```
  b
  ```
- c"##);
}

#[test]
fn markdown_lists_319() {
    // https://spec.commonmark.org/0.30/#example-319
    test_identical_markdown_events!(r##"- a
  - b

    c
- d"##);
}

#[test]
fn markdown_lists_320() {
    // https://spec.commonmark.org/0.30/#example-320
    test_identical_markdown_events!(r##"* a
  > b
  >
* c"##);
}

#[test]
fn markdown_lists_321() {
    // https://spec.commonmark.org/0.30/#example-321
    test_identical_markdown_events!(r##"- a
  > b
  ```
  c
  ```
- d"##);
}

#[test]
fn markdown_lists_322() {
    // https://spec.commonmark.org/0.30/#example-322
    test_identical_markdown_events!(r##"- a"##);
}

#[test]
fn markdown_lists_323() {
    // https://spec.commonmark.org/0.30/#example-323
    test_identical_markdown_events!(r##"- a
  - b"##);
}

#[test]
fn markdown_lists_324() {
    // https://spec.commonmark.org/0.30/#example-324
    test_identical_markdown_events!(r##"1. ```
   foo
   ```

   bar"##);
}

#[test]
fn markdown_lists_325() {
    // https://spec.commonmark.org/0.30/#example-325
    test_identical_markdown_events!(r##"* foo
  * bar

  baz"##);
}

#[test]
fn markdown_lists_326() {
    // https://spec.commonmark.org/0.30/#example-326
    test_identical_markdown_events!(r##"- a
  - b
  - c

- d
  - e
  - f"##);
}

#[test]
fn markdown_inlines_327() {
    // https://spec.commonmark.org/0.30/#example-327
    test_identical_markdown_events!(r##"`hi`lo`"##);
}

#[test]
fn markdown_code_spans_328() {
    // https://spec.commonmark.org/0.30/#example-328
    test_identical_markdown_events!(r##"`foo`"##);
}

#[test]
fn markdown_code_spans_329() {
    // https://spec.commonmark.org/0.30/#example-329
    test_identical_markdown_events!(r##"`` foo ` bar ``"##);
}

#[test]
fn markdown_code_spans_330() {
    // https://spec.commonmark.org/0.30/#example-330
    test_identical_markdown_events!(r##"` `` `"##);
}

#[test]
fn markdown_code_spans_331() {
    // https://spec.commonmark.org/0.30/#example-331
    test_identical_markdown_events!(r##"`  ``  `"##);
}

#[test]
fn markdown_code_spans_332() {
    // https://spec.commonmark.org/0.30/#example-332
    test_identical_markdown_events!(r##"` a`"##);
}

#[test]
fn markdown_code_spans_333() {
    // https://spec.commonmark.org/0.30/#example-333
    test_identical_markdown_events!(r##"` b `"##);
}

#[test]
fn markdown_code_spans_334() {
    // https://spec.commonmark.org/0.30/#example-334
    test_identical_markdown_events!(r##"` `
`  `"##);
}

#[test]
fn markdown_code_spans_335() {
    // https://spec.commonmark.org/0.30/#example-335
    test_identical_markdown_events!("``\nfoo\nbar  \nbaz\n``");
}

#[test]
fn markdown_code_spans_336() {
    // https://spec.commonmark.org/0.30/#example-336
    test_identical_markdown_events!("``\nfoo \n``");
}

#[test]
fn markdown_code_spans_337() {
    // https://spec.commonmark.org/0.30/#example-337
    test_identical_markdown_events!("`foo   bar \nbaz`");
}

#[test]
fn markdown_code_spans_338() {
    // https://spec.commonmark.org/0.30/#example-338
    test_identical_markdown_events!(r##"`foo\`bar`"##);
}

#[test]
fn markdown_code_spans_339() {
    // https://spec.commonmark.org/0.30/#example-339
    test_identical_markdown_events!(r##"``foo`bar``"##);
}

#[test]
fn markdown_code_spans_340() {
    // https://spec.commonmark.org/0.30/#example-340
    test_identical_markdown_events!(r##"` foo `` bar `"##);
}

#[test]
fn markdown_code_spans_341() {
    // https://spec.commonmark.org/0.30/#example-341
    test_identical_markdown_events!(r##"*foo`*`"##);
}

#[test]
fn markdown_code_spans_342() {
    // https://spec.commonmark.org/0.30/#example-342
    test_identical_markdown_events!(r##"[not a `link](/foo`)"##);
}

#[test]
fn markdown_code_spans_343() {
    // https://spec.commonmark.org/0.30/#example-343
    test_identical_markdown_events!(r##"`<a href="`">`"##);
}

#[test]
fn markdown_code_spans_344() {
    // https://spec.commonmark.org/0.30/#example-344
    test_identical_markdown_events!(r##"<a href="`">`"##);
}

#[test]
fn markdown_code_spans_345() {
    // https://spec.commonmark.org/0.30/#example-345
    test_identical_markdown_events!(r##"`<http://foo.bar.`baz>`"##);
}

#[test]
fn markdown_code_spans_346() {
    // https://spec.commonmark.org/0.30/#example-346
    test_identical_markdown_events!(r##"<http://foo.bar.`baz>`"##);
}

#[test]
fn markdown_code_spans_347() {
    // https://spec.commonmark.org/0.30/#example-347
    test_identical_markdown_events!(r##"```foo``"##);
}

#[test]
fn markdown_code_spans_348() {
    // https://spec.commonmark.org/0.30/#example-348
    test_identical_markdown_events!(r##"`foo"##);
}

#[test]
fn markdown_code_spans_349() {
    // https://spec.commonmark.org/0.30/#example-349
    test_identical_markdown_events!(r##"`foo``bar``"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_350() {
    // https://spec.commonmark.org/0.30/#example-350
    test_identical_markdown_events!(r##"*foo bar*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_351() {
    // https://spec.commonmark.org/0.30/#example-351
    test_identical_markdown_events!(r##"a * foo bar*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_352() {
    // https://spec.commonmark.org/0.30/#example-352
    test_identical_markdown_events!(r##"a*"foo"*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_353() {
    // https://spec.commonmark.org/0.30/#example-353
    test_identical_markdown_events!(r##"* a *"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_354() {
    // https://spec.commonmark.org/0.30/#example-354
    test_identical_markdown_events!(r##"foo*bar*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_355() {
    // https://spec.commonmark.org/0.30/#example-355
    test_identical_markdown_events!(r##"5*6*78"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_356() {
    // https://spec.commonmark.org/0.30/#example-356
    test_identical_markdown_events!(r##"_foo bar_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_357() {
    // https://spec.commonmark.org/0.30/#example-357
    test_identical_markdown_events!(r##"_ foo bar_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_358() {
    // https://spec.commonmark.org/0.30/#example-358
    test_identical_markdown_events!(r##"a_"foo"_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_359() {
    // https://spec.commonmark.org/0.30/#example-359
    test_identical_markdown_events!(r##"foo_bar_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_360() {
    // https://spec.commonmark.org/0.30/#example-360
    test_identical_markdown_events!(r##"5_6_78"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_361() {
    // https://spec.commonmark.org/0.30/#example-361
    test_identical_markdown_events!(r##"пристаням_стремятся_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_362() {
    // https://spec.commonmark.org/0.30/#example-362
    test_identical_markdown_events!(r##"aa_"bb"_cc"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_363() {
    // https://spec.commonmark.org/0.30/#example-363
    test_identical_markdown_events!(r##"foo-_(bar)_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_364() {
    // https://spec.commonmark.org/0.30/#example-364
    test_identical_markdown_events!(r##"_foo*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_365() {
    // https://spec.commonmark.org/0.30/#example-365
    test_identical_markdown_events!(r##"*foo bar *"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_366() {
    // https://spec.commonmark.org/0.30/#example-366
    test_identical_markdown_events!(r##"*foo bar
*"##,r##"*foo bar
*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_367() {
    // https://spec.commonmark.org/0.30/#example-367
    test_identical_markdown_events!(r##"*(*foo)"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_368() {
    // https://spec.commonmark.org/0.30/#example-368
    test_identical_markdown_events!(r##"*(*foo*)*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_369() {
    // https://spec.commonmark.org/0.30/#example-369
    test_identical_markdown_events!(r##"*foo*bar"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_370() {
    // https://spec.commonmark.org/0.30/#example-370
    test_identical_markdown_events!(r##"_foo bar _"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_371() {
    // https://spec.commonmark.org/0.30/#example-371
    test_identical_markdown_events!(r##"_(_foo)"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_372() {
    // https://spec.commonmark.org/0.30/#example-372
    test_identical_markdown_events!(r##"_(_foo_)_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_373() {
    // https://spec.commonmark.org/0.30/#example-373
    test_identical_markdown_events!(r##"_foo_bar"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_374() {
    // https://spec.commonmark.org/0.30/#example-374
    test_identical_markdown_events!(r##"_пристаням_стремятся"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_375() {
    // https://spec.commonmark.org/0.30/#example-375
    test_identical_markdown_events!(r##"_foo_bar_baz_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_376() {
    // https://spec.commonmark.org/0.30/#example-376
    test_identical_markdown_events!(r##"_(bar)_."##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_377() {
    // https://spec.commonmark.org/0.30/#example-377
    test_identical_markdown_events!(r##"**foo bar**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_378() {
    // https://spec.commonmark.org/0.30/#example-378
    test_identical_markdown_events!(r##"** foo bar**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_379() {
    // https://spec.commonmark.org/0.30/#example-379
    test_identical_markdown_events!(r##"a**"foo"**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_380() {
    // https://spec.commonmark.org/0.30/#example-380
    test_identical_markdown_events!(r##"foo**bar**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_381() {
    // https://spec.commonmark.org/0.30/#example-381
    test_identical_markdown_events!(r##"__foo bar__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_382() {
    // https://spec.commonmark.org/0.30/#example-382
    test_identical_markdown_events!(r##"__ foo bar__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_383() {
    // https://spec.commonmark.org/0.30/#example-383
    test_identical_markdown_events!(r##"__
foo bar__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_384() {
    // https://spec.commonmark.org/0.30/#example-384
    test_identical_markdown_events!(r##"a__"foo"__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_385() {
    // https://spec.commonmark.org/0.30/#example-385
    test_identical_markdown_events!(r##"foo__bar__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_386() {
    // https://spec.commonmark.org/0.30/#example-386
    test_identical_markdown_events!(r##"5__6__78"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_387() {
    // https://spec.commonmark.org/0.30/#example-387
    test_identical_markdown_events!(r##"пристаням__стремятся__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_388() {
    // https://spec.commonmark.org/0.30/#example-388
    test_identical_markdown_events!(r##"__foo, __bar__, baz__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_389() {
    // https://spec.commonmark.org/0.30/#example-389
    test_identical_markdown_events!(r##"foo-__(bar)__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_390() {
    // https://spec.commonmark.org/0.30/#example-390
    test_identical_markdown_events!(r##"**foo bar **"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_391() {
    // https://spec.commonmark.org/0.30/#example-391
    test_identical_markdown_events!(r##"**(**foo)"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_392() {
    // https://spec.commonmark.org/0.30/#example-392
    test_identical_markdown_events!(r##"*(**foo**)*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_393() {
    // https://spec.commonmark.org/0.30/#example-393
    test_identical_markdown_events!(r##"**Gomphocarpus (*Gomphocarpus physocarpus*, syn.
*Asclepias physocarpa*)**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_394() {
    // https://spec.commonmark.org/0.30/#example-394
    test_identical_markdown_events!(r##"**foo "*bar*" foo**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_395() {
    // https://spec.commonmark.org/0.30/#example-395
    test_identical_markdown_events!(r##"**foo**bar"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_396() {
    // https://spec.commonmark.org/0.30/#example-396
    test_identical_markdown_events!(r##"__foo bar __"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_397() {
    // https://spec.commonmark.org/0.30/#example-397
    test_identical_markdown_events!(r##"__(__foo)"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_398() {
    // https://spec.commonmark.org/0.30/#example-398
    test_identical_markdown_events!(r##"_(__foo__)_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_399() {
    // https://spec.commonmark.org/0.30/#example-399
    test_identical_markdown_events!(r##"__foo__bar"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_400() {
    // https://spec.commonmark.org/0.30/#example-400
    test_identical_markdown_events!(r##"__пристаням__стремятся"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_401() {
    // https://spec.commonmark.org/0.30/#example-401
    test_identical_markdown_events!(r##"__foo__bar__baz__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_402() {
    // https://spec.commonmark.org/0.30/#example-402
    test_identical_markdown_events!(r##"__(bar)__."##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_403() {
    // https://spec.commonmark.org/0.30/#example-403
    test_identical_markdown_events!(r##"*foo [bar](/url)*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_404() {
    // https://spec.commonmark.org/0.30/#example-404
    test_identical_markdown_events!(r##"*foo
bar*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_405() {
    // https://spec.commonmark.org/0.30/#example-405
    test_identical_markdown_events!(r##"_foo __bar__ baz_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_406() {
    // https://spec.commonmark.org/0.30/#example-406
    test_identical_markdown_events!(r##"_foo _bar_ baz_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_407() {
    // https://spec.commonmark.org/0.30/#example-407
    test_identical_markdown_events!(r##"__foo_ bar_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_408() {
    // https://spec.commonmark.org/0.30/#example-408
    test_identical_markdown_events!(r##"*foo *bar**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_409() {
    // https://spec.commonmark.org/0.30/#example-409
    test_identical_markdown_events!(r##"*foo **bar** baz*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_410() {
    // https://spec.commonmark.org/0.30/#example-410
    test_identical_markdown_events!(r##"*foo**bar**baz*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_411() {
    // https://spec.commonmark.org/0.30/#example-411
    test_identical_markdown_events!(r##"*foo**bar*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_412() {
    // https://spec.commonmark.org/0.30/#example-412
    test_identical_markdown_events!(r##"***foo** bar*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_413() {
    // https://spec.commonmark.org/0.30/#example-413
    test_identical_markdown_events!(r##"*foo **bar***"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_414() {
    // https://spec.commonmark.org/0.30/#example-414
    test_identical_markdown_events!(r##"*foo**bar***"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_415() {
    // https://spec.commonmark.org/0.30/#example-415
    test_identical_markdown_events!(r##"foo***bar***baz"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_416() {
    // https://spec.commonmark.org/0.30/#example-416
    test_identical_markdown_events!(r##"foo******bar*********baz"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_417() {
    // https://spec.commonmark.org/0.30/#example-417
    test_identical_markdown_events!(r##"*foo **bar *baz* bim** bop*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_418() {
    // https://spec.commonmark.org/0.30/#example-418
    test_identical_markdown_events!(r##"*foo [*bar*](/url)*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_419() {
    // https://spec.commonmark.org/0.30/#example-419
    test_identical_markdown_events!(r##"** is not an empty emphasis"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_420() {
    // https://spec.commonmark.org/0.30/#example-420
    test_identical_markdown_events!(r##"**** is not an empty strong emphasis"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_421() {
    // https://spec.commonmark.org/0.30/#example-421
    test_identical_markdown_events!(r##"**foo [bar](/url)**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_422() {
    // https://spec.commonmark.org/0.30/#example-422
    test_identical_markdown_events!(r##"**foo
bar**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_423() {
    // https://spec.commonmark.org/0.30/#example-423
    test_identical_markdown_events!(r##"__foo _bar_ baz__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_424() {
    // https://spec.commonmark.org/0.30/#example-424
    test_identical_markdown_events!(r##"__foo __bar__ baz__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_425() {
    // https://spec.commonmark.org/0.30/#example-425
    test_identical_markdown_events!(r##"____foo__ bar__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_426() {
    // https://spec.commonmark.org/0.30/#example-426
    test_identical_markdown_events!(r##"**foo **bar****"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_427() {
    // https://spec.commonmark.org/0.30/#example-427
    test_identical_markdown_events!(r##"**foo *bar* baz**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_428() {
    // https://spec.commonmark.org/0.30/#example-428
    test_identical_markdown_events!(r##"**foo*bar*baz**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_429() {
    // https://spec.commonmark.org/0.30/#example-429
    test_identical_markdown_events!(r##"***foo* bar**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_430() {
    // https://spec.commonmark.org/0.30/#example-430
    test_identical_markdown_events!(r##"**foo *bar***"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_431() {
    // https://spec.commonmark.org/0.30/#example-431
    test_identical_markdown_events!(r##"**foo *bar **baz**
bim* bop**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_432() {
    // https://spec.commonmark.org/0.30/#example-432
    test_identical_markdown_events!(r##"**foo [*bar*](/url)**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_433() {
    // https://spec.commonmark.org/0.30/#example-433
    test_identical_markdown_events!(r##"__ is not an empty emphasis"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_434() {
    // https://spec.commonmark.org/0.30/#example-434
    test_identical_markdown_events!(r##"____ is not an empty strong emphasis"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_435() {
    // https://spec.commonmark.org/0.30/#example-435
    test_identical_markdown_events!(r##"foo ***"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_436() {
    // https://spec.commonmark.org/0.30/#example-436
    test_identical_markdown_events!(r##"foo *\**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_437() {
    // https://spec.commonmark.org/0.30/#example-437
    test_identical_markdown_events!(r##"foo *_*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_438() {
    // https://spec.commonmark.org/0.30/#example-438
    test_identical_markdown_events!(r##"foo *****"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_439() {
    // https://spec.commonmark.org/0.30/#example-439
    test_identical_markdown_events!(r##"foo **\***"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_440() {
    // https://spec.commonmark.org/0.30/#example-440
    test_identical_markdown_events!(r##"foo **_**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_441() {
    // https://spec.commonmark.org/0.30/#example-441
    test_identical_markdown_events!(r##"**foo*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_442() {
    // https://spec.commonmark.org/0.30/#example-442
    test_identical_markdown_events!(r##"*foo**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_443() {
    // https://spec.commonmark.org/0.30/#example-443
    test_identical_markdown_events!(r##"***foo**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_444() {
    // https://spec.commonmark.org/0.30/#example-444
    test_identical_markdown_events!(r##"****foo*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_445() {
    // https://spec.commonmark.org/0.30/#example-445
    test_identical_markdown_events!(r##"**foo***"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_446() {
    // https://spec.commonmark.org/0.30/#example-446
    test_identical_markdown_events!(r##"*foo****"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_447() {
    // https://spec.commonmark.org/0.30/#example-447
    test_identical_markdown_events!(r##"foo ___"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_448() {
    // https://spec.commonmark.org/0.30/#example-448
    test_identical_markdown_events!(r##"foo _\__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_449() {
    // https://spec.commonmark.org/0.30/#example-449
    test_identical_markdown_events!(r##"foo _*_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_450() {
    // https://spec.commonmark.org/0.30/#example-450
    test_identical_markdown_events!(r##"foo _____"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_451() {
    // https://spec.commonmark.org/0.30/#example-451
    test_identical_markdown_events!(r##"foo __\___"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_452() {
    // https://spec.commonmark.org/0.30/#example-452
    test_identical_markdown_events!(r##"foo __*__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_453() {
    // https://spec.commonmark.org/0.30/#example-453
    test_identical_markdown_events!(r##"__foo_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_454() {
    // https://spec.commonmark.org/0.30/#example-454
    test_identical_markdown_events!(r##"_foo__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_455() {
    // https://spec.commonmark.org/0.30/#example-455
    test_identical_markdown_events!(r##"___foo__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_456() {
    // https://spec.commonmark.org/0.30/#example-456
    test_identical_markdown_events!(r##"____foo_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_457() {
    // https://spec.commonmark.org/0.30/#example-457
    test_identical_markdown_events!(r##"__foo___"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_458() {
    // https://spec.commonmark.org/0.30/#example-458
    test_identical_markdown_events!(r##"_foo____"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_459() {
    // https://spec.commonmark.org/0.30/#example-459
    test_identical_markdown_events!(r##"**foo**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_460() {
    // https://spec.commonmark.org/0.30/#example-460
    test_identical_markdown_events!(r##"*_foo_*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_461() {
    // https://spec.commonmark.org/0.30/#example-461
    test_identical_markdown_events!(r##"__foo__"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_462() {
    // https://spec.commonmark.org/0.30/#example-462
    test_identical_markdown_events!(r##"_*foo*_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_463() {
    // https://spec.commonmark.org/0.30/#example-463
    test_identical_markdown_events!(r##"****foo****"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_464() {
    // https://spec.commonmark.org/0.30/#example-464
    test_identical_markdown_events!(r##"____foo____"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_465() {
    // https://spec.commonmark.org/0.30/#example-465
    test_identical_markdown_events!(r##"******foo******"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_466() {
    // https://spec.commonmark.org/0.30/#example-466
    test_identical_markdown_events!(r##"***foo***"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_467() {
    // https://spec.commonmark.org/0.30/#example-467
    test_identical_markdown_events!(r##"_____foo_____"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_468() {
    // https://spec.commonmark.org/0.30/#example-468
    test_identical_markdown_events!(r##"*foo _bar* baz_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_469() {
    // https://spec.commonmark.org/0.30/#example-469
    test_identical_markdown_events!(r##"*foo __bar *baz bim__ bam*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_470() {
    // https://spec.commonmark.org/0.30/#example-470
    test_identical_markdown_events!(r##"**foo **bar baz**"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_471() {
    // https://spec.commonmark.org/0.30/#example-471
    test_identical_markdown_events!(r##"*foo *bar baz*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_472() {
    // https://spec.commonmark.org/0.30/#example-472
    test_identical_markdown_events!(r##"*[bar*](/url)"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_473() {
    // https://spec.commonmark.org/0.30/#example-473
    test_identical_markdown_events!(r##"_foo [bar_](/url)"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_474() {
    // https://spec.commonmark.org/0.30/#example-474
    test_identical_markdown_events!(r##"*<img src="foo" title="*"/>"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_475() {
    // https://spec.commonmark.org/0.30/#example-475
    test_identical_markdown_events!(r##"**<a href="**">"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_476() {
    // https://spec.commonmark.org/0.30/#example-476
    test_identical_markdown_events!(r##"__<a href="__">"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_477() {
    // https://spec.commonmark.org/0.30/#example-477
    test_identical_markdown_events!(r##"*a `*`*"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_478() {
    // https://spec.commonmark.org/0.30/#example-478
    test_identical_markdown_events!(r##"_a `_`_"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_479() {
    // https://spec.commonmark.org/0.30/#example-479
    test_identical_markdown_events!(r##"**a<http://foo.bar/?q=**>"##);
}

#[test]
fn markdown_emphasis_and_strong_emphasis_480() {
    // https://spec.commonmark.org/0.30/#example-480
    test_identical_markdown_events!(r##"__a<http://foo.bar/?q=__>"##);
}

#[test]
fn markdown_links_481() {
    // https://spec.commonmark.org/0.30/#example-481
    test_identical_markdown_events!(r##"[link](/uri "title")"##);
}

#[test]
fn markdown_links_482() {
    // https://spec.commonmark.org/0.30/#example-482
    test_identical_markdown_events!(r##"[link](/uri)"##);
}

#[test]
fn markdown_links_483() {
    // https://spec.commonmark.org/0.30/#example-483
    test_identical_markdown_events!(r##"[](./target.md)"##);
}

#[test]
fn markdown_links_484() {
    // https://spec.commonmark.org/0.30/#example-484
    test_identical_markdown_events!(r##"[link]()"##);
}

#[test]
fn markdown_links_485() {
    // https://spec.commonmark.org/0.30/#example-485
    test_identical_markdown_events!(r##"[link](<>)"##,r##"[link]()"##);
}

#[test]
fn markdown_links_486() {
    // https://spec.commonmark.org/0.30/#example-486
    test_identical_markdown_events!(r##"[]()"##);
}

#[test]
fn markdown_links_487() {
    // https://spec.commonmark.org/0.30/#example-487
    test_identical_markdown_events!(r##"[link](/my uri)"##);
}

#[test]
fn markdown_links_488() {
    // https://spec.commonmark.org/0.30/#example-488
    test_identical_markdown_events!(r##"[link](</my uri>)"##);
}

#[test]
fn markdown_links_489() {
    // https://spec.commonmark.org/0.30/#example-489
    test_identical_markdown_events!(r##"[link](foo
bar)"##);
}

#[test]
fn markdown_links_490() {
    // https://spec.commonmark.org/0.30/#example-490
    test_identical_markdown_events!(r##"[link](<foo
bar>)"##);
}

#[test]
fn markdown_links_491() {
    // https://spec.commonmark.org/0.30/#example-491
    test_identical_markdown_events!(r##"[a](<b)c>)"##);
}

#[test]
fn markdown_links_492() {
    // https://spec.commonmark.org/0.30/#example-492
    test_identical_markdown_events!(r##"[link](<foo\>)"##);
}

#[test]
fn markdown_links_493() {
    // https://spec.commonmark.org/0.30/#example-493
    test_identical_markdown_events!(r##"[a](<b)c
[a](<b)c>
[a](<b>c)"##);
}

#[test]
fn markdown_links_494() {
    // https://spec.commonmark.org/0.30/#example-494
    test_identical_markdown_events!(r##"[link](\(foo\))"##,r##"[link](\(foo\))"##);
}

#[test]
fn markdown_links_495() {
    // https://spec.commonmark.org/0.30/#example-495
    test_identical_markdown_events!(r##"[link](foo(and(bar)))"##);
}

#[test]
fn markdown_links_496() {
    // https://spec.commonmark.org/0.30/#example-496
    test_identical_markdown_events!(r##"[link](foo(and(bar))"##);
}

#[test]
fn markdown_links_497() {
    // https://spec.commonmark.org/0.30/#example-497
    test_identical_markdown_events!(r##"[link](foo\(and\(bar\))"##,r##"[link](foo\(and\(bar\))"##);
}

#[test]
fn markdown_links_498() {
    // https://spec.commonmark.org/0.30/#example-498
    test_identical_markdown_events!(r##"[link](<foo(and(bar)>)"##);
}

#[test]
fn markdown_links_499() {
    // https://spec.commonmark.org/0.30/#example-499
    test_identical_markdown_events!(r##"[link](foo\)\:)"##,r##"[link](foo\)\:)"##);
}

#[test]
fn markdown_links_500() {
    // https://spec.commonmark.org/0.30/#example-500
    test_identical_markdown_events!(r##"[link](#fragment)

[link](http://example.com#fragment)

[link](http://example.com?foo=3#frag)"##);
}

#[test]
fn markdown_links_501() {
    // https://spec.commonmark.org/0.30/#example-501
    test_identical_markdown_events!(r##"[link](foo\bar)"##);
}

#[test]
fn markdown_links_502() {
    // https://spec.commonmark.org/0.30/#example-502
    test_identical_markdown_events!(r##"[link](foo%20b&auml;)"##,r##"[link](foo%20b&auml;)"##);
}

#[test]
fn markdown_links_503() {
    // https://spec.commonmark.org/0.30/#example-503
    test_identical_markdown_events!(r##"[link]("title")"##);
}

#[test]
fn markdown_links_504() {
    // https://spec.commonmark.org/0.30/#example-504
    test_identical_markdown_events!(r##"[link](/url "title")
[link](/url 'title')
[link](/url (title))"##,r##"[link](/url "title")
[link](/url 'title')
[link](/url (title))"##);
}

#[test]
fn markdown_links_505() {
    // https://spec.commonmark.org/0.30/#example-505
    test_identical_markdown_events!(r##"[link](/url "title \"&quot;")"##);
}

#[test]
fn markdown_links_506() {
    // https://spec.commonmark.org/0.30/#example-506
    test!(r##"[link](/url "title")"##,r##"[link](/url "title")"##);
}

#[test]
fn markdown_links_507() {
    // https://spec.commonmark.org/0.30/#example-507
    test_identical_markdown_events!(r##"[link](/url "title "and" title")"##);
}

#[test]
fn markdown_links_508() {
    // https://spec.commonmark.org/0.30/#example-508
    test_identical_markdown_events!(r##"[link](/url 'title "and" title')"##);
}

#[test]
fn markdown_links_509() {
    // https://spec.commonmark.org/0.30/#example-509
    test_identical_markdown_events!(r##"[link](   /uri
  "title"  )"##,r##"[link](/uri "title")"##);
}

#[test]
fn markdown_links_510() {
    // https://spec.commonmark.org/0.30/#example-510
    test_identical_markdown_events!(r##"[link] (/uri)"##);
}

#[test]
fn markdown_links_511() {
    // https://spec.commonmark.org/0.30/#example-511
    test_identical_markdown_events!(r##"[link [foo [bar]]](/uri)"##);
}

#[test]
fn markdown_links_512() {
    // https://spec.commonmark.org/0.30/#example-512
    test_identical_markdown_events!(r##"[link] bar](/uri)"##);
}

#[test]
fn markdown_links_513() {
    // https://spec.commonmark.org/0.30/#example-513
    test_identical_markdown_events!(r##"[link [bar](/uri)"##);
}

#[test]
fn markdown_links_514() {
    // https://spec.commonmark.org/0.30/#example-514
    test_identical_markdown_events!(r##"[link \[bar](/uri)"##);
}

#[test]
fn markdown_links_515() {
    // https://spec.commonmark.org/0.30/#example-515
    test_identical_markdown_events!(r##"[link *foo **bar** `#`*](/uri)"##);
}

#[test]
fn markdown_links_516() {
    // https://spec.commonmark.org/0.30/#example-516
    test_identical_markdown_events!(r##"[![moon](moon.jpg)](/uri)"##);
}

#[test]
fn markdown_links_517() {
    // https://spec.commonmark.org/0.30/#example-517
    test_identical_markdown_events!(r##"[foo [bar](/uri)](/uri)"##);
}

#[test]
fn markdown_links_518() {
    // https://spec.commonmark.org/0.30/#example-518
    test_identical_markdown_events!(r##"[foo *[bar [baz](/uri)](/uri)*](/uri)"##);
}

#[test]
fn markdown_links_519() {
    // https://spec.commonmark.org/0.30/#example-519
    test_identical_markdown_events!(r##"![[[foo](uri1)](uri2)](uri3)"##);
}

#[test]
fn markdown_links_520() {
    // https://spec.commonmark.org/0.30/#example-520
    test_identical_markdown_events!(r##"*[foo*](/uri)"##);
}

#[test]
fn markdown_links_521() {
    // https://spec.commonmark.org/0.30/#example-521
    test_identical_markdown_events!(r##"[foo *bar](baz*)"##);
}

#[test]
fn markdown_links_522() {
    // https://spec.commonmark.org/0.30/#example-522
    test_identical_markdown_events!(r##"*foo [bar* baz]"##);
}

#[test]
fn markdown_links_523() {
    // https://spec.commonmark.org/0.30/#example-523
    test_identical_markdown_events!(r##"[foo <bar attr="](baz)">"##);
}

#[test]
fn markdown_links_524() {
    // https://spec.commonmark.org/0.30/#example-524
    test_identical_markdown_events!(r##"[foo`](/uri)`"##);
}

#[test]
fn markdown_links_525() {
    // https://spec.commonmark.org/0.30/#example-525
    test_identical_markdown_events!(r##"[foo<http://example.com/?search=](uri)>"##);
}

#[test]
fn markdown_links_526() {
    // https://spec.commonmark.org/0.30/#example-526
    test_identical_markdown_events!(r##"[foo][bar]

[bar]: /url "title""##);
}

#[test]
fn markdown_links_527() {
    // https://spec.commonmark.org/0.30/#example-527
    test_identical_markdown_events!(r##"[link [foo [bar]]][ref]

[ref]: /uri"##);
}

#[test]
fn markdown_links_528() {
    // https://spec.commonmark.org/0.30/#example-528
    test_identical_markdown_events!(r##"[link \[bar][ref]

[ref]: /uri"##);
}

#[test]
fn markdown_links_529() {
    // https://spec.commonmark.org/0.30/#example-529
    test_identical_markdown_events!(r##"[link *foo **bar** `#`*][ref]

[ref]: /uri"##);
}

#[test]
fn markdown_links_530() {
    // https://spec.commonmark.org/0.30/#example-530
    test_identical_markdown_events!(r##"[![moon](moon.jpg)][ref]

[ref]: /uri"##);
}

#[test]
fn markdown_links_531() {
    // https://spec.commonmark.org/0.30/#example-531
    test_identical_markdown_events!(r##"[foo [bar](/uri)][ref]

[ref]: /uri"##);
}

#[test]
fn markdown_links_532() {
    // https://spec.commonmark.org/0.30/#example-532
    test_identical_markdown_events!(r##"[foo *bar [baz][ref]*][ref]

[ref]: /uri"##);
}

#[test]
fn markdown_links_533() {
    // https://spec.commonmark.org/0.30/#example-533
    test_identical_markdown_events!(r##"*[foo*][ref]

[ref]: /uri"##);
}

#[test]
fn markdown_links_534() {
    // https://spec.commonmark.org/0.30/#example-534
    test_identical_markdown_events!(r##"[foo *bar][ref]*

[ref]: /uri"##);
}

#[test]
fn markdown_links_535() {
    // https://spec.commonmark.org/0.30/#example-535
    test_identical_markdown_events!(r##"[foo <bar attr="][ref]">

[ref]: /uri"##);
}

#[test]
fn markdown_links_536() {
    // https://spec.commonmark.org/0.30/#example-536
    test_identical_markdown_events!(r##"[foo`][ref]`

[ref]: /uri"##);
}

#[test]
fn markdown_links_537() {
    // https://spec.commonmark.org/0.30/#example-537
    test_identical_markdown_events!(r##"[foo<http://example.com/?search=][ref]>

[ref]: /uri"##);
}

#[test]
fn markdown_links_538() {
    // https://spec.commonmark.org/0.30/#example-538
    test_identical_markdown_events!(r##"[foo][BaR]

[bar]: /url "title""##,r##"[foo][BaR]

[bar]: /url "title""##);
}

#[test]
fn markdown_links_539() {
    // https://spec.commonmark.org/0.30/#example-539
    test_identical_markdown_events!(r##"[ẞ]

[SS]: /url"##);
}

#[test]
fn markdown_links_540() {
    // https://spec.commonmark.org/0.30/#example-540
    test_identical_markdown_events!(r##"[Foo
  bar]: /url

[Baz][Foo bar]"##,r##"[Foo bar]: /url

[Baz][Foo bar]"##);
}

#[test]
fn markdown_links_541() {
    // https://spec.commonmark.org/0.30/#example-541
    test_identical_markdown_events!(r##"[foo] [bar]

[bar]: /url "title""##);
}

#[test]
fn markdown_links_542() {
    // https://spec.commonmark.org/0.30/#example-542
    test_identical_markdown_events!(r##"[foo]
[bar]

[bar]: /url "title""##);
}

#[test]
fn markdown_links_543() {
    // https://spec.commonmark.org/0.30/#example-543
    test!(r##"[foo]: /url1

[foo]: /url2

[bar][foo]"##,r##"[foo]: /url1



[bar][foo]"##);
}

#[test]
fn markdown_links_544() {
    // https://spec.commonmark.org/0.30/#example-544
    test_identical_markdown_events!(r##"[bar][foo\!]

[foo!]: /url"##);
}

#[test]
fn markdown_links_545() {
    // https://spec.commonmark.org/0.30/#example-545
    test_identical_markdown_events!(r##"[foo][ref[]

[ref[]: /uri"##);
}

#[test]
fn markdown_links_546() {
    // https://spec.commonmark.org/0.30/#example-546
    test_identical_markdown_events!(r##"[foo][ref[bar]]

[ref[bar]]: /uri"##);
}

#[test]
fn markdown_links_547() {
    // https://spec.commonmark.org/0.30/#example-547
    test_identical_markdown_events!(r##"[[[foo]]]

[[[foo]]]: /url"##);
}

#[test]
fn markdown_links_548() {
    // https://spec.commonmark.org/0.30/#example-548
    test_identical_markdown_events!(r##"[foo][ref\[]

[ref\[]: /uri"##);
}

#[test]
fn markdown_links_549() {
    // https://spec.commonmark.org/0.30/#example-549
    test_identical_markdown_events!(r##"[bar\\]: /uri

[bar\\]"##,r##"[bar\\]: /uri

[bar\\]"##);
}

#[test]
fn markdown_links_550() {
    // https://spec.commonmark.org/0.30/#example-550
    test_identical_markdown_events!(r##"[]

[]: /uri"##);
}

#[test]
fn markdown_links_551() {
    // https://spec.commonmark.org/0.30/#example-551
    test_identical_markdown_events!(r##"[
 ]

[
 ]: /uri"##,r##"[
]

[
]: /uri"##);
}

#[test]
fn markdown_links_552() {
    // https://spec.commonmark.org/0.30/#example-552
    test_identical_markdown_events!(r##"[foo][]

[foo]: /url "title""##);
}

#[test]
fn markdown_links_553() {
    // https://spec.commonmark.org/0.30/#example-553
    test_identical_markdown_events!(r##"[*foo* bar][]

[*foo* bar]: /url "title""##);
}

#[test]
fn markdown_links_554() {
    // https://spec.commonmark.org/0.30/#example-554
    test_identical_markdown_events!(r##"[Foo][]

[foo]: /url "title""##);
}

#[test]
fn markdown_links_555() {
    // https://spec.commonmark.org/0.30/#example-555
    test_identical_markdown_events!("[foo] \n[]\n\n[foo]: /url \"title\"");
}

#[test]
fn markdown_links_556() {
    // https://spec.commonmark.org/0.30/#example-556
    test_identical_markdown_events!(r##"[foo]

[foo]: /url "title""##);
}

#[test]
fn markdown_links_557() {
    // https://spec.commonmark.org/0.30/#example-557
    test_identical_markdown_events!(r##"[*foo* bar]

[*foo* bar]: /url "title""##);
}

#[test]
fn markdown_links_558() {
    // https://spec.commonmark.org/0.30/#example-558
    test_identical_markdown_events!(r##"[[*foo* bar]]

[*foo* bar]: /url "title""##);
}

#[test]
fn markdown_links_559() {
    // https://spec.commonmark.org/0.30/#example-559
    test_identical_markdown_events!(r##"[[bar [foo]

[foo]: /url"##);
}

#[test]
fn markdown_links_560() {
    // https://spec.commonmark.org/0.30/#example-560
    test_identical_markdown_events!(r##"[Foo]

[foo]: /url "title""##);
}

#[test]
fn markdown_links_561() {
    // https://spec.commonmark.org/0.30/#example-561
    test_identical_markdown_events!(r##"[foo] bar

[foo]: /url"##);
}

#[test]
fn markdown_links_562() {
    // https://spec.commonmark.org/0.30/#example-562
    test_identical_markdown_events!(r##"\[foo]

[foo]: /url "title""##);
}

#[test]
fn markdown_links_563() {
    // https://spec.commonmark.org/0.30/#example-563
    test_identical_markdown_events!(r##"[foo*]: /url

*[foo*]"##,r##"[foo*]: /url

*[foo*]"##);
}

#[test]
fn markdown_links_564() {
    // https://spec.commonmark.org/0.30/#example-564
    test_identical_markdown_events!(r##"[foo][bar]

[foo]: /url1
[bar]: /url2"##);
}

#[test]
fn markdown_links_565() {
    // https://spec.commonmark.org/0.30/#example-565
    test_identical_markdown_events!(r##"[foo][]

[foo]: /url1"##);
}

#[test]
fn markdown_links_566() {
    // https://spec.commonmark.org/0.30/#example-566
    test_identical_markdown_events!(r##"[foo]()

[foo]: /url1"##);
}

#[test]
fn markdown_links_567() {
    // https://spec.commonmark.org/0.30/#example-567
    test_identical_markdown_events!(r##"[foo](not a link)

[foo]: /url1"##);
}

#[test]
fn markdown_links_568() {
    // https://spec.commonmark.org/0.30/#example-568
    test_identical_markdown_events!(r##"[foo][bar][baz]

[baz]: /url"##);
}

#[test]
fn markdown_links_569() {
    // https://spec.commonmark.org/0.30/#example-569
    test_identical_markdown_events!(r##"[foo][bar][baz]

[baz]: /url1
[bar]: /url2"##);
}

#[test]
fn markdown_links_570() {
    // https://spec.commonmark.org/0.30/#example-570
    test_identical_markdown_events!(r##"[foo][bar][baz]

[baz]: /url1
[foo]: /url2"##);
}

#[test]
fn markdown_images_571() {
    // https://spec.commonmark.org/0.30/#example-571
    test_identical_markdown_events!(r##"![foo](/url "title")"##);
}

#[test]
fn markdown_images_572() {
    // https://spec.commonmark.org/0.30/#example-572
    test_identical_markdown_events!(r##"![foo *bar*]

[foo *bar*]: train.jpg "train & tracks""##);
}

#[test]
fn markdown_images_573() {
    // https://spec.commonmark.org/0.30/#example-573
    test_identical_markdown_events!(r##"![foo ![bar](/url)](/url2)"##);
}

#[test]
fn markdown_images_574() {
    // https://spec.commonmark.org/0.30/#example-574
    test_identical_markdown_events!(r##"![foo [bar](/url)](/url2)"##);
}

#[test]
fn markdown_images_575() {
    // https://spec.commonmark.org/0.30/#example-575
    test_identical_markdown_events!(r##"![foo *bar*][]

[foo *bar*]: train.jpg "train & tracks""##);
}

#[test]
fn markdown_images_576() {
    // https://spec.commonmark.org/0.30/#example-576
    test_identical_markdown_events!(r##"![foo *bar*][foobar]

[FOOBAR]: train.jpg "train & tracks""##,r##"![foo *bar*][foobar]

[FOOBAR]: train.jpg "train & tracks""##);
}

#[test]
fn markdown_images_577() {
    // https://spec.commonmark.org/0.30/#example-577
    test_identical_markdown_events!(r##"![foo](train.jpg)"##);
}

#[test]
fn markdown_images_578() {
    // https://spec.commonmark.org/0.30/#example-578
    test_identical_markdown_events!(r##"My ![foo bar](/path/to/train.jpg  "title"   )"##,r##"My ![foo bar](/path/to/train.jpg "title")"##);
}

#[test]
fn markdown_images_579() {
    // https://spec.commonmark.org/0.30/#example-579
    test_identical_markdown_events!(r##"![foo](<url>)"##,r##"![foo](url)"##);
}

#[test]
fn markdown_images_580() {
    // https://spec.commonmark.org/0.30/#example-580
    test_identical_markdown_events!(r##"![](/url)"##);
}

#[test]
fn markdown_images_581() {
    // https://spec.commonmark.org/0.30/#example-581
    test_identical_markdown_events!(r##"![foo][bar]

[bar]: /url"##);
}

#[test]
fn markdown_images_582() {
    // https://spec.commonmark.org/0.30/#example-582
    test_identical_markdown_events!(r##"![foo][bar]

[BAR]: /url"##,r##"![foo][bar]

[BAR]: /url"##);
}

#[test]
fn markdown_images_583() {
    // https://spec.commonmark.org/0.30/#example-583
    test_identical_markdown_events!(r##"![foo][]

[foo]: /url "title""##);
}

#[test]
fn markdown_images_584() {
    // https://spec.commonmark.org/0.30/#example-584
    test_identical_markdown_events!(r##"![*foo* bar][]

[*foo* bar]: /url "title""##);
}

#[test]
fn markdown_images_585() {
    // https://spec.commonmark.org/0.30/#example-585
    test_identical_markdown_events!(r##"![Foo][]

[foo]: /url "title""##);
}

#[test]
fn markdown_images_586() {
    // https://spec.commonmark.org/0.30/#example-586
    test_identical_markdown_events!("![foo] \n[]\n\n[foo]: /url \"title\"");
}

#[test]
fn markdown_images_587() {
    // https://spec.commonmark.org/0.30/#example-587
    test_identical_markdown_events!(r##"![foo]

[foo]: /url "title""##);
}

#[test]
fn markdown_images_588() {
    // https://spec.commonmark.org/0.30/#example-588
    test_identical_markdown_events!(r##"![*foo* bar]

[*foo* bar]: /url "title""##);
}

#[test]
fn markdown_images_589() {
    // https://spec.commonmark.org/0.30/#example-589
    test_identical_markdown_events!(r##"![[foo]]

[[foo]]: /url "title""##);
}

#[test]
fn markdown_images_590() {
    // https://spec.commonmark.org/0.30/#example-590
    test_identical_markdown_events!(r##"![Foo]

[foo]: /url "title""##);
}

#[test]
fn markdown_images_591() {
    // https://spec.commonmark.org/0.30/#example-591
    test_identical_markdown_events!(r##"!\[foo]

[foo]: /url "title""##);
}

#[test]
fn markdown_images_592() {
    // https://spec.commonmark.org/0.30/#example-592
    test_identical_markdown_events!(r##"\![foo]

[foo]: /url "title""##);
}

#[test]
fn markdown_autolinks_593() {
    // https://spec.commonmark.org/0.30/#example-593
    test_identical_markdown_events!(r##"<http://foo.bar.baz>"##);
}

#[test]
fn markdown_autolinks_594() {
    // https://spec.commonmark.org/0.30/#example-594
    test_identical_markdown_events!(r##"<http://foo.bar.baz/test?q=hello&id=22&boolean>"##);
}

#[test]
fn markdown_autolinks_595() {
    // https://spec.commonmark.org/0.30/#example-595
    test_identical_markdown_events!(r##"<irc://foo.bar:2233/baz>"##);
}

#[test]
fn markdown_autolinks_596() {
    // https://spec.commonmark.org/0.30/#example-596
    test_identical_markdown_events!(r##"<MAILTO:FOO@BAR.BAZ>"##);
}

#[test]
fn markdown_autolinks_597() {
    // https://spec.commonmark.org/0.30/#example-597
    test_identical_markdown_events!(r##"<a+b+c:d>"##);
}

#[test]
fn markdown_autolinks_598() {
    // https://spec.commonmark.org/0.30/#example-598
    test_identical_markdown_events!(r##"<made-up-scheme://foo,bar>"##);
}

#[test]
fn markdown_autolinks_599() {
    // https://spec.commonmark.org/0.30/#example-599
    test_identical_markdown_events!(r##"<http://../>"##);
}

#[test]
fn markdown_autolinks_600() {
    // https://spec.commonmark.org/0.30/#example-600
    test_identical_markdown_events!(r##"<localhost:5001/foo>"##);
}

#[test]
fn markdown_autolinks_601() {
    // https://spec.commonmark.org/0.30/#example-601
    test_identical_markdown_events!(r##"<http://foo.bar/baz bim>"##);
}

#[test]
fn markdown_autolinks_602() {
    // https://spec.commonmark.org/0.30/#example-602
    test_identical_markdown_events!(r##"<http://example.com/\[\>"##);
}

#[test]
fn markdown_autolinks_603() {
    // https://spec.commonmark.org/0.30/#example-603
    test_identical_markdown_events!(r##"<foo@bar.example.com>"##);
}

#[test]
fn markdown_autolinks_604() {
    // https://spec.commonmark.org/0.30/#example-604
    test_identical_markdown_events!(r##"<foo+special@Bar.baz-bar0.com>"##);
}

#[test]
fn markdown_autolinks_605() {
    // https://spec.commonmark.org/0.30/#example-605
    test_identical_markdown_events!(r##"<foo\+@bar.example.com>"##);
}

#[test]
fn markdown_autolinks_606() {
    // https://spec.commonmark.org/0.30/#example-606
    test_identical_markdown_events!(r##"<>"##);
}

#[test]
fn markdown_autolinks_607() {
    // https://spec.commonmark.org/0.30/#example-607
    test_identical_markdown_events!(r##"< http://foo.bar >"##);
}

#[test]
fn markdown_autolinks_608() {
    // https://spec.commonmark.org/0.30/#example-608
    test_identical_markdown_events!(r##"<m:abc>"##);
}

#[test]
fn markdown_autolinks_609() {
    // https://spec.commonmark.org/0.30/#example-609
    test_identical_markdown_events!(r##"<foo.bar.baz>"##);
}

#[test]
fn markdown_autolinks_610() {
    // https://spec.commonmark.org/0.30/#example-610
    test_identical_markdown_events!(r##"http://example.com"##);
}

#[test]
fn markdown_autolinks_611() {
    // https://spec.commonmark.org/0.30/#example-611
    test_identical_markdown_events!(r##"foo@bar.example.com"##);
}

#[test]
fn markdown_raw_html_612() {
    // https://spec.commonmark.org/0.30/#example-612
    test_identical_markdown_events!(r##"<a><bab><c2c>"##);
}

#[test]
fn markdown_raw_html_613() {
    // https://spec.commonmark.org/0.30/#example-613
    test_identical_markdown_events!(r##"<a/><b2/>"##);
}

#[test]
fn markdown_raw_html_614() {
    // https://spec.commonmark.org/0.30/#example-614
    test_identical_markdown_events!(r##"<a  /><b2
data="foo" >"##);
}

#[test]
fn markdown_raw_html_615() {
    // https://spec.commonmark.org/0.30/#example-615
    test_identical_markdown_events!(r##"<a foo="bar" bam = 'baz <em>"</em>'
_boolean zoop:33=zoop:33 />"##);
}

#[test]
fn markdown_raw_html_616() {
    // https://spec.commonmark.org/0.30/#example-616
    test_identical_markdown_events!(r##"Foo <responsive-image src="foo.jpg" />"##);
}

#[test]
fn markdown_raw_html_617() {
    // https://spec.commonmark.org/0.30/#example-617
    test_identical_markdown_events!(r##"<33> <__>"##);
}

#[test]
fn markdown_raw_html_618() {
    // https://spec.commonmark.org/0.30/#example-618
    test_identical_markdown_events!(r##"<a h*#ref="hi">"##);
}

#[test]
fn markdown_raw_html_619() {
    // https://spec.commonmark.org/0.30/#example-619
    test_identical_markdown_events!(r##"<a href="hi'> <a href=hi'>"##);
}

#[test]
fn markdown_raw_html_620() {
    // https://spec.commonmark.org/0.30/#example-620
    test_identical_markdown_events!(r##"< a><
foo><bar/ >
<foo bar=baz
bim!bop />"##);
}

#[test]
fn markdown_raw_html_621() {
    // https://spec.commonmark.org/0.30/#example-621
    test_identical_markdown_events!(r##"<a href='bar'title=title>"##);
}

#[test]
fn markdown_raw_html_622() {
    // https://spec.commonmark.org/0.30/#example-622
    test_identical_markdown_events!(r##"</a></foo >"##);
}

#[test]
fn markdown_raw_html_623() {
    // https://spec.commonmark.org/0.30/#example-623
    test_identical_markdown_events!(r##"</a href="foo">"##);
}

#[test]
fn markdown_raw_html_624() {
    // https://spec.commonmark.org/0.30/#example-624
    test_identical_markdown_events!(r##"foo <!-- this is a
comment - with hyphen -->"##);
}

#[test]
fn markdown_raw_html_625() {
    // https://spec.commonmark.org/0.30/#example-625
    test_identical_markdown_events!(r##"foo <!-- not a comment -- two hyphens -->"##);
}

#[test]
fn markdown_raw_html_626() {
    // https://spec.commonmark.org/0.30/#example-626
    test_identical_markdown_events!(r##"foo <!--> foo -->

foo <!-- foo--->"##);
}

#[test]
fn markdown_raw_html_627() {
    // https://spec.commonmark.org/0.30/#example-627
    test_identical_markdown_events!(r##"foo <?php echo $a; ?>"##);
}

#[test]
fn markdown_raw_html_628() {
    // https://spec.commonmark.org/0.30/#example-628
    test_identical_markdown_events!(r##"foo <!ELEMENT br EMPTY>"##);
}

#[test]
fn markdown_raw_html_629() {
    // https://spec.commonmark.org/0.30/#example-629
    test_identical_markdown_events!(r##"foo <![CDATA[>&<]]>"##);
}

#[test]
fn markdown_raw_html_630() {
    // https://spec.commonmark.org/0.30/#example-630
    test_identical_markdown_events!(r##"foo <a href="&ouml;">"##);
}

#[test]
fn markdown_raw_html_631() {
    // https://spec.commonmark.org/0.30/#example-631
    test_identical_markdown_events!(r##"foo <a href="\*">"##);
}

#[test]
fn markdown_raw_html_632() {
    // https://spec.commonmark.org/0.30/#example-632
    test_identical_markdown_events!(r##"<a href="\"">"##);
}

#[test]
fn markdown_hard_line_breaks_633() {
    // https://spec.commonmark.org/0.30/#example-633
    test_identical_markdown_events!("foo  \nbaz");
}

#[test]
fn markdown_hard_line_breaks_634() {
    // https://spec.commonmark.org/0.30/#example-634
    test_identical_markdown_events!(r##"foo\
baz"##);
}

#[test]
fn markdown_hard_line_breaks_635() {
    // https://spec.commonmark.org/0.30/#example-635
    test_identical_markdown_events!("foo       \nbaz");
}

#[test]
fn markdown_hard_line_breaks_636() {
    // https://spec.commonmark.org/0.30/#example-636
    test_identical_markdown_events!("foo  \n     bar","foo  \nbar");
}

#[test]
fn markdown_hard_line_breaks_637() {
    // https://spec.commonmark.org/0.30/#example-637
    test_identical_markdown_events!(r##"foo\
     bar"##,r##"foo\
bar"##);
}

#[test]
fn markdown_hard_line_breaks_638() {
    // https://spec.commonmark.org/0.30/#example-638
    test_identical_markdown_events!("*foo  \nbar*");
}

#[test]
fn markdown_hard_line_breaks_639() {
    // https://spec.commonmark.org/0.30/#example-639
    test_identical_markdown_events!(r##"*foo\
bar*"##);
}

#[test]
fn markdown_hard_line_breaks_640() {
    // https://spec.commonmark.org/0.30/#example-640
    test_identical_markdown_events!("`code  \nspan`");
}

#[test]
fn markdown_hard_line_breaks_641() {
    // https://spec.commonmark.org/0.30/#example-641
    test_identical_markdown_events!(r##"`code\
span`"##);
}

#[test]
fn markdown_hard_line_breaks_642() {
    // https://spec.commonmark.org/0.30/#example-642
    test_identical_markdown_events!("<a href=\"foo  \nbar\">");
}

#[test]
fn markdown_hard_line_breaks_643() {
    // https://spec.commonmark.org/0.30/#example-643
    test_identical_markdown_events!(r##"<a href="foo\
bar">"##);
}

#[test]
fn markdown_hard_line_breaks_644() {
    // https://spec.commonmark.org/0.30/#example-644
    test_identical_markdown_events!(r##"foo\"##);
}

#[test]
fn markdown_hard_line_breaks_645() {
    // https://spec.commonmark.org/0.30/#example-645
    test_identical_markdown_events!("foo  ");
}

#[test]
fn markdown_hard_line_breaks_646() {
    // https://spec.commonmark.org/0.30/#example-646
    test_identical_markdown_events!(r##"### foo\"##);
}

#[test]
fn markdown_hard_line_breaks_647() {
    // https://spec.commonmark.org/0.30/#example-647
    test_identical_markdown_events!("### foo  ",r##"### foo"##);
}

#[test]
fn markdown_soft_line_breaks_648() {
    // https://spec.commonmark.org/0.30/#example-648
    test_identical_markdown_events!(r##"foo
baz"##);
}

#[test]
fn markdown_soft_line_breaks_649() {
    // https://spec.commonmark.org/0.30/#example-649
    test_identical_markdown_events!("foo \n baz","foo \nbaz");
}

#[test]
fn markdown_textual_content_650() {
    // https://spec.commonmark.org/0.30/#example-650
    test_identical_markdown_events!(r##"hello $.;'there"##);
}

#[test]
fn markdown_textual_content_651() {
    // https://spec.commonmark.org/0.30/#example-651
    test_identical_markdown_events!(r##"Foo χρῆν"##);
}

#[test]
fn markdown_textual_content_652() {
    // https://spec.commonmark.org/0.30/#example-652
    test_identical_markdown_events!(r##"Multiple     spaces"##);
}
