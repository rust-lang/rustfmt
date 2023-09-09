// @generated
// generated running `cargo build -F gen-tests`
// test macros are defined in tests/common/mod.rs
mod common;

#[test]
fn gfm_markdown_tabs_1() {
    // https://github.github.com/gfm/#example-1
    test_identical_markdown_events!(r##"	foo	baz		bim"##,r##"    foo	baz		bim"##);
}

#[test]
fn gfm_markdown_tabs_2() {
    // https://github.github.com/gfm/#example-2
    test_identical_markdown_events!(r##"  	foo	baz		bim"##,r##"    foo	baz		bim"##);
}

#[test]
fn gfm_markdown_tabs_3() {
    // https://github.github.com/gfm/#example-3
    test_identical_markdown_events!(r##"    a	a
    ὐ	a"##);
}

#[test]
fn gfm_markdown_tabs_4() {
    // https://github.github.com/gfm/#example-4
    test_identical_markdown_events!(r##"  - foo

	bar"##,r##"- foo

  bar"##);
}

#[test]
fn gfm_markdown_tabs_5() {
    // https://github.github.com/gfm/#example-5
    test_identical_markdown_events!(r##"- foo

		bar"##,r##"- foo

        bar"##);
}

#[test]
fn gfm_markdown_tabs_6() {
    // https://github.github.com/gfm/#example-6
    test_identical_markdown_events!(r##">		foo"##,r##">       foo"##);
}

#[test]
fn gfm_markdown_tabs_7() {
    // https://github.github.com/gfm/#example-7
    test_identical_markdown_events!(r##"-		foo"##,r##"-       foo"##);
}

#[test]
fn gfm_markdown_tabs_8() {
    // https://github.github.com/gfm/#example-8
    test_identical_markdown_events!(r##"    foo
	bar"##,r##"    foo
    bar"##);
}

#[test]
fn gfm_markdown_tabs_9() {
    // https://github.github.com/gfm/#example-9
    test_identical_markdown_events!(r##" - foo
   - bar
	 - baz"##,r##"- foo
  - bar
    - baz"##);
}

#[test]
fn gfm_markdown_tabs_10() {
    // https://github.github.com/gfm/#example-10
    test_identical_markdown_events!(r##"#	Foo"##,r##"# Foo"##);
}

#[test]
fn gfm_markdown_tabs_11() {
    // https://github.github.com/gfm/#example-11
    test_identical_markdown_events!("*\t*\t*\t");
}

#[test]
fn gfm_markdown_precedence_12() {
    // https://github.github.com/gfm/#example-12
    test_identical_markdown_events!(r##"- `one
- two`"##);
}

#[test]
fn gfm_markdown_thematic_breaks_13() {
    // https://github.github.com/gfm/#example-13
    test_identical_markdown_events!(r##"***
---
___"##);
}

#[test]
fn gfm_markdown_thematic_breaks_14() {
    // https://github.github.com/gfm/#example-14
    test_identical_markdown_events!(r##"+++"##);
}

#[test]
fn gfm_markdown_thematic_breaks_15() {
    // https://github.github.com/gfm/#example-15
    test_identical_markdown_events!(r##"==="##);
}

#[test]
fn gfm_markdown_thematic_breaks_16() {
    // https://github.github.com/gfm/#example-16
    test_identical_markdown_events!(r##"--
**
__"##);
}

#[test]
fn gfm_markdown_thematic_breaks_17() {
    // https://github.github.com/gfm/#example-17
    test_identical_markdown_events!(r##" ***
  ***
   ***"##,r##"***
***
***"##);
}

#[test]
fn gfm_markdown_thematic_breaks_18() {
    // https://github.github.com/gfm/#example-18
    test_identical_markdown_events!(r##"    ***"##);
}

#[test]
fn gfm_markdown_thematic_breaks_19() {
    // https://github.github.com/gfm/#example-19
    test!(r##"Foo
    ***"##,r##"Foo
\***"##);
}

#[test]
fn gfm_markdown_thematic_breaks_20() {
    // https://github.github.com/gfm/#example-20
    test_identical_markdown_events!(r##"_____________________________________"##);
}

#[test]
fn gfm_markdown_thematic_breaks_21() {
    // https://github.github.com/gfm/#example-21
    test_identical_markdown_events!(r##" - - -"##,r##"- - -"##);
}

#[test]
fn gfm_markdown_thematic_breaks_22() {
    // https://github.github.com/gfm/#example-22
    test_identical_markdown_events!(r##" **  * ** * ** * **"##,r##"**  * ** * ** * **"##);
}

#[test]
fn gfm_markdown_thematic_breaks_23() {
    // https://github.github.com/gfm/#example-23
    test_identical_markdown_events!(r##"-     -      -      -"##);
}

#[test]
fn gfm_markdown_thematic_breaks_24() {
    // https://github.github.com/gfm/#example-24
    test_identical_markdown_events!("- - - -    ");
}

#[test]
fn gfm_markdown_thematic_breaks_25() {
    // https://github.github.com/gfm/#example-25
    test_identical_markdown_events!(r##"_ _ _ _ a

a------

---a---"##);
}

#[test]
fn gfm_markdown_thematic_breaks_26() {
    // https://github.github.com/gfm/#example-26
    test_identical_markdown_events!(r##" *-*"##,r##"*-*"##);
}

#[test]
fn gfm_markdown_thematic_breaks_27() {
    // https://github.github.com/gfm/#example-27
    test_identical_markdown_events!(r##"- foo
***
- bar"##);
}

#[test]
fn gfm_markdown_thematic_breaks_28() {
    // https://github.github.com/gfm/#example-28
    test_identical_markdown_events!(r##"Foo
***
bar"##);
}

#[test]
fn gfm_markdown_thematic_breaks_29() {
    // https://github.github.com/gfm/#example-29
    test_identical_markdown_events!(r##"Foo
---
bar"##);
}

#[test]
fn gfm_markdown_thematic_breaks_30() {
    // https://github.github.com/gfm/#example-30
    test_identical_markdown_events!(r##"* Foo
* * *
* Bar"##);
}

#[test]
fn gfm_markdown_thematic_breaks_31() {
    // https://github.github.com/gfm/#example-31
    test_identical_markdown_events!(r##"- Foo
- * * *"##);
}

#[test]
fn gfm_markdown_atx_headings_32() {
    // https://github.github.com/gfm/#example-32
    test_identical_markdown_events!(r##"# foo
## foo
### foo
#### foo
##### foo
###### foo"##);
}

#[test]
fn gfm_markdown_atx_headings_33() {
    // https://github.github.com/gfm/#example-33
    test_identical_markdown_events!(r##"####### foo"##);
}

#[test]
fn gfm_markdown_atx_headings_34() {
    // https://github.github.com/gfm/#example-34
    test_identical_markdown_events!(r##"#5 bolt

#hashtag"##);
}

#[test]
fn gfm_markdown_atx_headings_35() {
    // https://github.github.com/gfm/#example-35
    test_identical_markdown_events!(r##"\## foo"##);
}

#[test]
fn gfm_markdown_atx_headings_36() {
    // https://github.github.com/gfm/#example-36
    test_identical_markdown_events!(r##"# foo *bar* \*baz\*"##);
}

#[test]
fn gfm_markdown_atx_headings_37() {
    // https://github.github.com/gfm/#example-37
    test_identical_markdown_events!("#                  foo                     ",r##"# foo"##);
}

#[test]
fn gfm_markdown_atx_headings_38() {
    // https://github.github.com/gfm/#example-38
    test_identical_markdown_events!(r##" ### foo
  ## foo
   # foo"##,r##"### foo
## foo
# foo"##);
}

#[test]
fn gfm_markdown_atx_headings_39() {
    // https://github.github.com/gfm/#example-39
    test_identical_markdown_events!(r##"    # foo"##);
}

#[test]
fn gfm_markdown_atx_headings_40() {
    // https://github.github.com/gfm/#example-40
    test_identical_markdown_events!(r##"foo
    # bar"##,r##"foo
\# bar"##);
}

#[test]
fn gfm_markdown_atx_headings_41() {
    // https://github.github.com/gfm/#example-41
    test_identical_markdown_events!(r##"## foo ##
  ###   bar    ###"##,r##"## foo
### bar"##);
}

#[test]
fn gfm_markdown_atx_headings_42() {
    // https://github.github.com/gfm/#example-42
    test_identical_markdown_events!(r##"# foo ##################################
##### foo ##"##,r##"# foo
##### foo"##);
}

#[test]
fn gfm_markdown_atx_headings_43() {
    // https://github.github.com/gfm/#example-43
    test_identical_markdown_events!("### foo ###     ",r##"### foo"##);
}

#[test]
fn gfm_markdown_atx_headings_44() {
    // https://github.github.com/gfm/#example-44
    test_identical_markdown_events!(r##"### foo ### b"##);
}

#[test]
fn gfm_markdown_atx_headings_45() {
    // https://github.github.com/gfm/#example-45
    test_identical_markdown_events!(r##"# foo#"##);
}

#[test]
fn gfm_markdown_atx_headings_46() {
    // https://github.github.com/gfm/#example-46
    test_identical_markdown_events!(r##"### foo \###
## foo #\##
# foo \#"##);
}

#[test]
fn gfm_markdown_atx_headings_47() {
    // https://github.github.com/gfm/#example-47
    test_identical_markdown_events!(r##"****
## foo
****"##);
}

#[test]
fn gfm_markdown_atx_headings_48() {
    // https://github.github.com/gfm/#example-48
    test_identical_markdown_events!(r##"Foo bar
# baz
Bar foo"##);
}

#[test]
fn gfm_markdown_atx_headings_49() {
    // https://github.github.com/gfm/#example-49
    test_identical_markdown_events!("## \n#\n### ###",r##"##
#
###"##);
}

#[test]
fn gfm_markdown_setext_headings_50() {
    // https://github.github.com/gfm/#example-50
    test_identical_markdown_events!(r##"Foo *bar*
=========

Foo *bar*
---------"##);
}

#[test]
fn gfm_markdown_setext_headings_51() {
    // https://github.github.com/gfm/#example-51
    test_identical_markdown_events!(r##"Foo *bar
baz*
===="##);
}

#[test]
fn gfm_markdown_setext_headings_52() {
    // https://github.github.com/gfm/#example-52
    test_identical_markdown_events!("  Foo *bar\nbaz*\t\n====",r##"Foo *bar
baz*
===="##);
}

#[test]
fn gfm_markdown_setext_headings_53() {
    // https://github.github.com/gfm/#example-53
    test_identical_markdown_events!(r##"Foo
-------------------------

Foo
="##);
}

#[test]
fn gfm_markdown_setext_headings_54() {
    // https://github.github.com/gfm/#example-54
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
fn gfm_markdown_setext_headings_55() {
    // https://github.github.com/gfm/#example-55
    test_identical_markdown_events!(r##"    Foo
    ---

    Foo
---"##);
}

#[test]
fn gfm_markdown_setext_headings_56() {
    // https://github.github.com/gfm/#example-56
    test_identical_markdown_events!("Foo\n   ----      ",r##"Foo
----"##);
}

#[test]
fn gfm_markdown_setext_headings_57() {
    // https://github.github.com/gfm/#example-57
    test_identical_markdown_events!(r##"Foo
    ---"##,r##"Foo
\---"##);
}

#[test]
fn gfm_markdown_setext_headings_58() {
    // https://github.github.com/gfm/#example-58
    test_identical_markdown_events!(r##"Foo
= =

Foo
--- -"##);
}

#[test]
fn gfm_markdown_setext_headings_59() {
    // https://github.github.com/gfm/#example-59
    test_identical_markdown_events!("Foo  \n-----",r##"Foo
-----"##);
}

#[test]
fn gfm_markdown_setext_headings_60() {
    // https://github.github.com/gfm/#example-60
    test_identical_markdown_events!(r##"Foo\
----"##);
}

#[test]
fn gfm_markdown_setext_headings_61() {
    // https://github.github.com/gfm/#example-61
    test_identical_markdown_events!(r##"`Foo
----
`

<a title="a lot
---
of dashes"/>"##);
}

#[test]
fn gfm_markdown_setext_headings_62() {
    // https://github.github.com/gfm/#example-62
    test_identical_markdown_events!(r##"> Foo
---"##);
}

#[test]
fn gfm_markdown_setext_headings_63() {
    // https://github.github.com/gfm/#example-63
    test_identical_markdown_events!(r##"> foo
bar
==="##,r##"> foo
> bar
> \==="##);
}

#[test]
fn gfm_markdown_setext_headings_64() {
    // https://github.github.com/gfm/#example-64
    test_identical_markdown_events!(r##"- Foo
---"##);
}

#[test]
fn gfm_markdown_setext_headings_65() {
    // https://github.github.com/gfm/#example-65
    test_identical_markdown_events!(r##"Foo
Bar
---"##);
}

#[test]
fn gfm_markdown_setext_headings_66() {
    // https://github.github.com/gfm/#example-66
    test_identical_markdown_events!(r##"---
Foo
---
Bar
---
Baz"##);
}

#[test]
fn gfm_markdown_setext_headings_67() {
    // https://github.github.com/gfm/#example-67
    test_identical_markdown_events!(r##"
===="##,r##"===="##);
}

#[test]
fn gfm_markdown_setext_headings_68() {
    // https://github.github.com/gfm/#example-68
    test_identical_markdown_events!(r##"---
---"##);
}

#[test]
fn gfm_markdown_setext_headings_69() {
    // https://github.github.com/gfm/#example-69
    test_identical_markdown_events!(r##"- foo
-----"##);
}

#[test]
fn gfm_markdown_setext_headings_70() {
    // https://github.github.com/gfm/#example-70
    test_identical_markdown_events!(r##"    foo
---"##);
}

#[test]
fn gfm_markdown_setext_headings_71() {
    // https://github.github.com/gfm/#example-71
    test_identical_markdown_events!(r##"> foo
-----"##);
}

#[test]
fn gfm_markdown_setext_headings_72() {
    // https://github.github.com/gfm/#example-72
    test_identical_markdown_events!(r##"\> foo
------"##);
}

#[test]
fn gfm_markdown_setext_headings_73() {
    // https://github.github.com/gfm/#example-73
    test_identical_markdown_events!(r##"Foo

bar
---
baz"##);
}

#[test]
fn gfm_markdown_setext_headings_74() {
    // https://github.github.com/gfm/#example-74
    test_identical_markdown_events!(r##"Foo
bar

---

baz"##);
}

#[test]
fn gfm_markdown_setext_headings_75() {
    // https://github.github.com/gfm/#example-75
    test_identical_markdown_events!(r##"Foo
bar
* * *
baz"##);
}

#[test]
fn gfm_markdown_setext_headings_76() {
    // https://github.github.com/gfm/#example-76
    test_identical_markdown_events!(r##"Foo
bar
\---
baz"##);
}

#[test]
fn gfm_markdown_indented_code_blocks_77() {
    // https://github.github.com/gfm/#example-77
    test_identical_markdown_events!(r##"    a simple
      indented code block"##);
}

#[test]
fn gfm_markdown_indented_code_blocks_78() {
    // https://github.github.com/gfm/#example-78
    test_identical_markdown_events!(r##"  - foo

    bar"##,r##"- foo

  bar"##);
}

#[test]
fn gfm_markdown_indented_code_blocks_79() {
    // https://github.github.com/gfm/#example-79
    test_identical_markdown_events!(r##"1.  foo

    - bar"##,r##"1. foo

   - bar"##);
}

#[test]
fn gfm_markdown_indented_code_blocks_80() {
    // https://github.github.com/gfm/#example-80
    test_identical_markdown_events!(r##"    <a/>
    *hi*

    - one"##);
}

#[test]
fn gfm_markdown_indented_code_blocks_81() {
    // https://github.github.com/gfm/#example-81
    test_identical_markdown_events!("    chunk1\n\n    chunk2\n  \n \n \n    chunk3",r##"    chunk1

    chunk2



    chunk3"##);
}

#[test]
fn gfm_markdown_indented_code_blocks_82() {
    // https://github.github.com/gfm/#example-82
    test_identical_markdown_events!("    chunk1\n      \n      chunk2",r##"    chunk1

      chunk2"##);
}

#[test]
fn gfm_markdown_indented_code_blocks_83() {
    // https://github.github.com/gfm/#example-83
    test_identical_markdown_events!(r##"Foo
    bar"##,r##"Foo
bar"##);
}

#[test]
fn gfm_markdown_indented_code_blocks_84() {
    // https://github.github.com/gfm/#example-84
    test_identical_markdown_events!(r##"    foo
bar"##);
}

#[test]
fn gfm_markdown_indented_code_blocks_85() {
    // https://github.github.com/gfm/#example-85
    test_identical_markdown_events!(r##"# Heading
    foo
Heading
------
    foo
----"##);
}

#[test]
fn gfm_markdown_indented_code_blocks_86() {
    // https://github.github.com/gfm/#example-86
    test_identical_markdown_events!(r##"        foo
    bar"##);
}

#[test]
fn gfm_markdown_indented_code_blocks_87() {
    // https://github.github.com/gfm/#example-87
    test_identical_markdown_events!("\n    \n    foo\n    ",r##"    foo"##);
}

#[test]
fn gfm_markdown_indented_code_blocks_88() {
    // https://github.github.com/gfm/#example-88
    test_identical_markdown_events!("    foo  ",r##"    foo"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_89() {
    // https://github.github.com/gfm/#example-89
    test_identical_markdown_events!(r##"```
<
 >
```"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_90() {
    // https://github.github.com/gfm/#example-90
    test_identical_markdown_events!(r##"~~~
<
 >
~~~"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_91() {
    // https://github.github.com/gfm/#example-91
    test_identical_markdown_events!(r##"``
foo
``"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_92() {
    // https://github.github.com/gfm/#example-92
    test_identical_markdown_events!(r##"```
aaa
~~~
```"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_93() {
    // https://github.github.com/gfm/#example-93
    test_identical_markdown_events!(r##"~~~
aaa
```
~~~"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_94() {
    // https://github.github.com/gfm/#example-94
    test_identical_markdown_events!(r##"````
aaa
```
``````"##,r##"````
aaa
```
````"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_95() {
    // https://github.github.com/gfm/#example-95
    test_identical_markdown_events!(r##"~~~~
aaa
~~~
~~~~"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_96() {
    // https://github.github.com/gfm/#example-96
    test_identical_markdown_events!(r##"```"##,r##"```
```"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_97() {
    // https://github.github.com/gfm/#example-97
    test_identical_markdown_events!(r##"`````

```
aaa"##,r##"`````

```
aaa
`````"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_98() {
    // https://github.github.com/gfm/#example-98
    test_identical_markdown_events!(r##"> ```
> aaa

bbb"##,r##"> ```
> aaa
> ```

bbb"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_99() {
    // https://github.github.com/gfm/#example-99
    test_identical_markdown_events!("```\n\n  \n```",r##"```
```"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_100() {
    // https://github.github.com/gfm/#example-100
    test_identical_markdown_events!(r##"```
```"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_101() {
    // https://github.github.com/gfm/#example-101
    test_identical_markdown_events!(r##" ```
 aaa
aaa
```"##,r##"```
aaa
aaa
```"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_102() {
    // https://github.github.com/gfm/#example-102
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
fn gfm_markdown_fenced_code_blocks_103() {
    // https://github.github.com/gfm/#example-103
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
fn gfm_markdown_fenced_code_blocks_104() {
    // https://github.github.com/gfm/#example-104
    test_identical_markdown_events!(r##"    ```
    aaa
    ```"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_105() {
    // https://github.github.com/gfm/#example-105
    test_identical_markdown_events!(r##"```
aaa
  ```"##,r##"```
aaa
```"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_106() {
    // https://github.github.com/gfm/#example-106
    test_identical_markdown_events!(r##"   ```
aaa
  ```"##,r##"```
aaa
```"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_107() {
    // https://github.github.com/gfm/#example-107
    test_identical_markdown_events!(r##"```
aaa
    ```"##,r##"```
aaa
    ```
```"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_108() {
    // https://github.github.com/gfm/#example-108
    test_identical_markdown_events!(r##"``` ```
aaa"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_109() {
    // https://github.github.com/gfm/#example-109
    test_identical_markdown_events!(r##"~~~~~~
aaa
~~~ ~~"##,r##"~~~~~~
aaa
~~~ ~~
~~~~~~"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_110() {
    // https://github.github.com/gfm/#example-110
    test_identical_markdown_events!(r##"foo
```
bar
```
baz"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_111() {
    // https://github.github.com/gfm/#example-111
    test_identical_markdown_events!(r##"foo
---
~~~
bar
~~~
# baz"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_112() {
    // https://github.github.com/gfm/#example-112
    test_identical_markdown_events!(r##"```ruby
def foo(x)
  return 3
end
```"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_113() {
    // https://github.github.com/gfm/#example-113
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
fn gfm_markdown_fenced_code_blocks_114() {
    // https://github.github.com/gfm/#example-114
    test_identical_markdown_events!(r##"````;
````"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_115() {
    // https://github.github.com/gfm/#example-115
    test_identical_markdown_events!(r##"``` aa ```
foo"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_116() {
    // https://github.github.com/gfm/#example-116
    test_identical_markdown_events!(r##"~~~ aa ``` ~~~
foo
~~~"##);
}

#[test]
fn gfm_markdown_fenced_code_blocks_117() {
    // https://github.github.com/gfm/#example-117
    test_identical_markdown_events!(r##"```
``` aaa
```"##);
}

#[test]
fn gfm_markdown_html_blocks_118() {
    // https://github.github.com/gfm/#example-118
    test_identical_markdown_events!(r##"<table><tr><td>
<pre>
**Hello**,

_world_.
</pre>
</td></tr></table>"##);
}

#[test]
fn gfm_markdown_html_blocks_119() {
    // https://github.github.com/gfm/#example-119
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
fn gfm_markdown_html_blocks_120() {
    // https://github.github.com/gfm/#example-120
    test_identical_markdown_events!(r##" <div>
  *hello*
         <foo><a>"##,r##"<div>
  *hello*
         <foo><a>"##);
}

#[test]
fn gfm_markdown_html_blocks_121() {
    // https://github.github.com/gfm/#example-121
    test_identical_markdown_events!(r##"</div>
*foo*"##);
}

#[test]
fn gfm_markdown_html_blocks_122() {
    // https://github.github.com/gfm/#example-122
    test_identical_markdown_events!(r##"<DIV CLASS="foo">

*Markdown*

</DIV>"##);
}

#[test]
fn gfm_markdown_html_blocks_123() {
    // https://github.github.com/gfm/#example-123
    test_identical_markdown_events!(r##"<div id="foo"
  class="bar">
</div>"##);
}

#[test]
fn gfm_markdown_html_blocks_124() {
    // https://github.github.com/gfm/#example-124
    test_identical_markdown_events!(r##"<div id="foo" class="bar
  baz">
</div>"##);
}

#[test]
fn gfm_markdown_html_blocks_125() {
    // https://github.github.com/gfm/#example-125
    test_identical_markdown_events!(r##"<div>
*foo*

*bar*"##);
}

#[test]
fn gfm_markdown_html_blocks_126() {
    // https://github.github.com/gfm/#example-126
    test_identical_markdown_events!(r##"<div id="foo"
*hi*"##);
}

#[test]
fn gfm_markdown_html_blocks_127() {
    // https://github.github.com/gfm/#example-127
    test_identical_markdown_events!(r##"<div class
foo"##);
}

#[test]
fn gfm_markdown_html_blocks_128() {
    // https://github.github.com/gfm/#example-128
    test_identical_markdown_events!(r##"<div *???-&&&-<---
*foo*"##);
}

#[test]
fn gfm_markdown_html_blocks_129() {
    // https://github.github.com/gfm/#example-129
    test_identical_markdown_events!(r##"<div><a href="bar">*foo*</a></div>"##);
}

#[test]
fn gfm_markdown_html_blocks_130() {
    // https://github.github.com/gfm/#example-130
    test_identical_markdown_events!(r##"<table><tr><td>
foo
</td></tr></table>"##);
}

#[test]
fn gfm_markdown_html_blocks_131() {
    // https://github.github.com/gfm/#example-131
    test_identical_markdown_events!(r##"<div></div>
``` c
int x = 33;
```"##);
}

#[test]
fn gfm_markdown_html_blocks_132() {
    // https://github.github.com/gfm/#example-132
    test_identical_markdown_events!(r##"<a href="foo">
*bar*
</a>"##);
}

#[test]
fn gfm_markdown_html_blocks_133() {
    // https://github.github.com/gfm/#example-133
    test_identical_markdown_events!(r##"<Warning>
*bar*
</Warning>"##);
}

#[test]
fn gfm_markdown_html_blocks_134() {
    // https://github.github.com/gfm/#example-134
    test_identical_markdown_events!(r##"<i class="foo">
*bar*
</i>"##);
}

#[test]
fn gfm_markdown_html_blocks_135() {
    // https://github.github.com/gfm/#example-135
    test_identical_markdown_events!(r##"</ins>
*bar*"##);
}

#[test]
fn gfm_markdown_html_blocks_136() {
    // https://github.github.com/gfm/#example-136
    test_identical_markdown_events!(r##"<del>
*foo*
</del>"##);
}

#[test]
fn gfm_markdown_html_blocks_137() {
    // https://github.github.com/gfm/#example-137
    test_identical_markdown_events!(r##"<del>

*foo*

</del>"##);
}

#[test]
fn gfm_markdown_html_blocks_138() {
    // https://github.github.com/gfm/#example-138
    test_identical_markdown_events!(r##"<del>*foo*</del>"##);
}

#[test]
fn gfm_markdown_html_blocks_139() {
    // https://github.github.com/gfm/#example-139
    test_identical_markdown_events!(r##"<pre language="haskell"><code>
import Text.HTML.TagSoup

main :: IO ()
main = print $ parseTags tags
</code></pre>
okay"##);
}

#[test]
fn gfm_markdown_html_blocks_140() {
    // https://github.github.com/gfm/#example-140
    test_identical_markdown_events!(r##"<script type="text/javascript">
// JavaScript example

document.getElementById("demo").innerHTML = "Hello JavaScript!";
</script>
okay"##);
}

#[test]
fn gfm_markdown_html_blocks_141() {
    // https://github.github.com/gfm/#example-141
    test_identical_markdown_events!(r##"<style
  type="text/css">
h1 {color:red;}

p {color:blue;}
</style>
okay"##);
}

#[test]
fn gfm_markdown_html_blocks_142() {
    // https://github.github.com/gfm/#example-142
    test_identical_markdown_events!(r##"<style
  type="text/css">

foo"##);
}

#[test]
fn gfm_markdown_html_blocks_143() {
    // https://github.github.com/gfm/#example-143
    test_identical_markdown_events!(r##"> <div>
> foo

bar"##);
}

#[test]
fn gfm_markdown_html_blocks_144() {
    // https://github.github.com/gfm/#example-144
    test_identical_markdown_events!(r##"- <div>
- foo"##);
}

#[test]
fn gfm_markdown_html_blocks_145() {
    // https://github.github.com/gfm/#example-145
    test_identical_markdown_events!(r##"<style>p{color:red;}</style>
*foo*"##);
}

#[test]
fn gfm_markdown_html_blocks_146() {
    // https://github.github.com/gfm/#example-146
    test_identical_markdown_events!(r##"<!-- foo -->*bar*
*baz*"##);
}

#[test]
fn gfm_markdown_html_blocks_147() {
    // https://github.github.com/gfm/#example-147
    test_identical_markdown_events!(r##"<script>
foo
</script>1. *bar*"##);
}

#[test]
fn gfm_markdown_html_blocks_148() {
    // https://github.github.com/gfm/#example-148
    test_identical_markdown_events!(r##"<!-- Foo

bar
   baz -->
okay"##);
}

#[test]
fn gfm_markdown_html_blocks_149() {
    // https://github.github.com/gfm/#example-149
    test_identical_markdown_events!(r##"<?php

  echo '>';

?>
okay"##);
}

#[test]
fn gfm_markdown_html_blocks_150() {
    // https://github.github.com/gfm/#example-150
    test_identical_markdown_events!(r##"<!DOCTYPE html>"##);
}

#[test]
fn gfm_markdown_html_blocks_151() {
    // https://github.github.com/gfm/#example-151
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
fn gfm_markdown_html_blocks_152() {
    // https://github.github.com/gfm/#example-152
    test_identical_markdown_events!(r##"  <!-- foo -->

    <!-- foo -->"##,r##"<!-- foo -->

    <!-- foo -->"##);
}

#[test]
fn gfm_markdown_html_blocks_153() {
    // https://github.github.com/gfm/#example-153
    test_identical_markdown_events!(r##"  <div>

    <div>"##,r##"<div>

    <div>"##);
}

#[test]
fn gfm_markdown_html_blocks_154() {
    // https://github.github.com/gfm/#example-154
    test_identical_markdown_events!(r##"Foo
<div>
bar
</div>"##);
}

#[test]
fn gfm_markdown_html_blocks_155() {
    // https://github.github.com/gfm/#example-155
    test_identical_markdown_events!(r##"<div>
bar
</div>
*foo*"##);
}

#[test]
fn gfm_markdown_html_blocks_156() {
    // https://github.github.com/gfm/#example-156
    test_identical_markdown_events!(r##"Foo
<a href="bar">
baz"##);
}

#[test]
fn gfm_markdown_html_blocks_157() {
    // https://github.github.com/gfm/#example-157
    test_identical_markdown_events!(r##"<div>

*Emphasized* text.

</div>"##);
}

#[test]
fn gfm_markdown_html_blocks_158() {
    // https://github.github.com/gfm/#example-158
    test_identical_markdown_events!(r##"<div>
*Emphasized* text.
</div>"##);
}

#[test]
fn gfm_markdown_html_blocks_159() {
    // https://github.github.com/gfm/#example-159
    test_identical_markdown_events!(r##"<table>

<tr>

<td>
Hi
</td>

</tr>

</table>"##);
}

#[test]
fn gfm_markdown_html_blocks_160() {
    // https://github.github.com/gfm/#example-160
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
fn gfm_markdown_link_reference_definitions_161() {
    // https://github.github.com/gfm/#example-161
    test_identical_markdown_events!(r##"[foo]: /url "title"

[foo]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_162() {
    // https://github.github.com/gfm/#example-162
    test_identical_markdown_events!("   [foo]: \n      /url  \n           'the title'  \n\n[foo]",r##"[foo]: /url 'the title'

[foo]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_163() {
    // https://github.github.com/gfm/#example-163
    test_identical_markdown_events!(r##"[Foo*bar\]]:my_(url) 'title (with parens)'

[Foo*bar\]]"##,r##"[Foo*bar\]]: my_(url) 'title (with parens)'

[Foo*bar\]]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_164() {
    // https://github.github.com/gfm/#example-164
    test_identical_markdown_events!(r##"[Foo bar]:
<my url>
'title'

[Foo bar]"##,r##"[Foo bar]: <my url> 'title'

[Foo bar]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_165() {
    // https://github.github.com/gfm/#example-165
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
fn gfm_markdown_link_reference_definitions_166() {
    // https://github.github.com/gfm/#example-166
    test_identical_markdown_events!(r##"[foo]: /url 'title

with blank line'

[foo]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_167() {
    // https://github.github.com/gfm/#example-167
    test_identical_markdown_events!(r##"[foo]:
/url

[foo]"##,r##"[foo]: /url

[foo]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_168() {
    // https://github.github.com/gfm/#example-168
    test_identical_markdown_events!(r##"[foo]:

[foo]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_169() {
    // https://github.github.com/gfm/#example-169
    test_identical_markdown_events!(r##"[foo]: <>

[foo]"##,r##"[foo]: <>

[foo]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_170() {
    // https://github.github.com/gfm/#example-170
    test_identical_markdown_events!(r##"[foo]: <bar>(baz)

[foo]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_171() {
    // https://github.github.com/gfm/#example-171
    test_identical_markdown_events!(r##"[foo]: /url\bar\*baz "foo\"bar\baz"

[foo]"##,r##"[foo]: /url\bar\*baz "foo\"bar\baz"

[foo]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_172() {
    // https://github.github.com/gfm/#example-172
    test_identical_markdown_events!(r##"[foo]

[foo]: url"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_173() {
    // https://github.github.com/gfm/#example-173
    test_identical_markdown_events!(r##"[foo]

[foo]: first
[foo]: second"##,r##"[foo]

[foo]: first"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_174() {
    // https://github.github.com/gfm/#example-174
    test_identical_markdown_events!(r##"[FOO]: /url

[Foo]"##,r##"[FOO]: /url

[Foo]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_175() {
    // https://github.github.com/gfm/#example-175
    test_identical_markdown_events!(r##"[ΑΓΩ]: /φου

[αγω]"##,r##"[ΑΓΩ]: /φου

[αγω]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_176() {
    // https://github.github.com/gfm/#example-176
    test_identical_markdown_events!(r##"[foo]: /url"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_177() {
    // https://github.github.com/gfm/#example-177
    test_identical_markdown_events!(r##"[
foo
]: /url
bar"##,r##"[ foo ]: /url
bar"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_178() {
    // https://github.github.com/gfm/#example-178
    test_identical_markdown_events!(r##"[foo]: /url "title" ok"##);
}

// FIXME(ytmim) the "title" is duplcated here
#[ignore]
#[test]
fn gfm_markdown_link_reference_definitions_179() {
    // https://github.github.com/gfm/#example-179
    test_identical_markdown_events!(r##"[foo]: /url
"title" ok"##,r##"[foo]: /url "title""title" ok"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_180() {
    // https://github.github.com/gfm/#example-180
    test_identical_markdown_events!(r##"    [foo]: /url "title"

[foo]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_181() {
    // https://github.github.com/gfm/#example-181
    test_identical_markdown_events!(r##"```
[foo]: /url
```

[foo]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_182() {
    // https://github.github.com/gfm/#example-182
    test_identical_markdown_events!(r##"Foo
[bar]: /baz

[bar]"##,r##"Foo
[bar]: /baz

[bar]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_183() {
    // https://github.github.com/gfm/#example-183
    test_identical_markdown_events!(r##"# [Foo]
[foo]: /url
> bar"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_184() {
    // https://github.github.com/gfm/#example-184
    test_identical_markdown_events!(r##"[foo]: /url
bar
===
[foo]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_185() {
    // https://github.github.com/gfm/#example-185
    test_identical_markdown_events!(r##"[foo]: /url
===
[foo]"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_186() {
    // https://github.github.com/gfm/#example-186
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
fn gfm_markdown_link_reference_definitions_187() {
    // https://github.github.com/gfm/#example-187
    test!(r##"[foo]

> [foo]: /url"##,r##"[foo]

>
[foo]: /url"##);
}

#[test]
fn gfm_markdown_link_reference_definitions_188() {
    // https://github.github.com/gfm/#example-188
    test_identical_markdown_events!(r##"[foo]: /url"##);
}

#[test]
fn gfm_markdown_paragraphs_189() {
    // https://github.github.com/gfm/#example-189
    test_identical_markdown_events!(r##"aaa

bbb"##);
}

#[test]
fn gfm_markdown_paragraphs_190() {
    // https://github.github.com/gfm/#example-190
    test_identical_markdown_events!(r##"aaa
bbb

ccc
ddd"##);
}

#[test]
fn gfm_markdown_paragraphs_191() {
    // https://github.github.com/gfm/#example-191
    test_identical_markdown_events!(r##"aaa


bbb"##);
}

#[test]
fn gfm_markdown_paragraphs_192() {
    // https://github.github.com/gfm/#example-192
    test_identical_markdown_events!(r##"  aaa
 bbb"##,r##"aaa
bbb"##);
}

#[test]
fn gfm_markdown_paragraphs_193() {
    // https://github.github.com/gfm/#example-193
    test_identical_markdown_events!(r##"aaa
             bbb
                                       ccc"##,r##"aaa
bbb
ccc"##);
}

#[test]
fn gfm_markdown_paragraphs_194() {
    // https://github.github.com/gfm/#example-194
    test_identical_markdown_events!(r##"   aaa
bbb"##,r##"aaa
bbb"##);
}

#[test]
fn gfm_markdown_paragraphs_195() {
    // https://github.github.com/gfm/#example-195
    test_identical_markdown_events!(r##"    aaa
bbb"##);
}

#[test]
fn gfm_markdown_paragraphs_196() {
    // https://github.github.com/gfm/#example-196
    test_identical_markdown_events!("aaa     \nbbb     ");
}

#[test]
fn gfm_markdown_blank_lines_197() {
    // https://github.github.com/gfm/#example-197
    test_identical_markdown_events!("  \n\naaa\n  \n\n# aaa\n\n  ",r##"aaa


# aaa"##);
}

#[test]
fn gfm_markdown_tables_extension_198() {
    // https://github.github.com/gfm/#example-198
    test_identical_markdown_events!(r##"| foo | bar |
| --- | --- |
| baz | bim |"##);
}

#[test]
fn gfm_markdown_tables_extension_199() {
    // https://github.github.com/gfm/#example-199
    test_identical_markdown_events!(r##"| abc | defghi |
:-: | -----------:
bar | baz"##,r##"| abc | defghi |
| :-: | -----: |
| bar | baz    |"##);
}

#[test]
fn gfm_markdown_tables_extension_200() {
    // https://github.github.com/gfm/#example-200
    test_identical_markdown_events!(r##"| f\|oo  |
| ------ |
| b `\|` az |
| b **\|** im |"##,r##"| f\|oo       |
| ----------- |
| b `\|` az   |
| b **\|** im |"##);
}

#[test]
fn gfm_markdown_tables_extension_201() {
    // https://github.github.com/gfm/#example-201
    test_identical_markdown_events!(r##"| abc | def |
| --- | --- |
| bar | baz |
> bar"##);
}

#[test]
fn gfm_markdown_tables_extension_202() {
    // https://github.github.com/gfm/#example-202
    test_identical_markdown_events!(r##"| abc | def |
| --- | --- |
| bar | baz |
bar

bar"##,r##"| abc | def |
| --- | --- |
| bar | baz |
| bar |     |

bar"##);
}

#[test]
fn gfm_markdown_tables_extension_203() {
    // https://github.github.com/gfm/#example-203
    test_identical_markdown_events!(r##"| abc | def |
| --- |
| bar |"##);
}

#[test]
fn gfm_markdown_tables_extension_204() {
    // https://github.github.com/gfm/#example-204
    test_identical_markdown_events!(r##"| abc | def |
| --- | --- |
| bar |
| bar | baz | boo |"##,r##"| abc | def |
| --- | --- |
| bar |     |
| bar | baz |"##);
}

#[test]
fn gfm_markdown_tables_extension_205() {
    // https://github.github.com/gfm/#example-205
    test_identical_markdown_events!(r##"| abc | def |
| --- | --- |"##);
}

#[test]
fn gfm_markdown_block_quotes_206() {
    // https://github.github.com/gfm/#example-206
    test_identical_markdown_events!(r##"> # Foo
> bar
> baz"##);
}

#[test]
fn gfm_markdown_block_quotes_207() {
    // https://github.github.com/gfm/#example-207
    test_identical_markdown_events!(r##"># Foo
>bar
> baz"##,r##"> # Foo
> bar
> baz"##);
}

#[test]
fn gfm_markdown_block_quotes_208() {
    // https://github.github.com/gfm/#example-208
    test_identical_markdown_events!(r##"   > # Foo
   > bar
 > baz"##,r##"> # Foo
> bar
> baz"##);
}

#[test]
fn gfm_markdown_block_quotes_209() {
    // https://github.github.com/gfm/#example-209
    test_identical_markdown_events!(r##"    > # Foo
    > bar
    > baz"##);
}

#[test]
fn gfm_markdown_block_quotes_210() {
    // https://github.github.com/gfm/#example-210
    test_identical_markdown_events!(r##"> # Foo
> bar
baz"##,r##"> # Foo
> bar
> baz"##);
}

#[test]
fn gfm_markdown_block_quotes_211() {
    // https://github.github.com/gfm/#example-211
    test_identical_markdown_events!(r##"> bar
baz
> foo"##,r##"> bar
> baz
> foo"##);
}

#[test]
fn gfm_markdown_block_quotes_212() {
    // https://github.github.com/gfm/#example-212
    test_identical_markdown_events!(r##"> foo
---"##);
}

#[test]
fn gfm_markdown_block_quotes_213() {
    // https://github.github.com/gfm/#example-213
    test_identical_markdown_events!(r##"> - foo
- bar"##);
}

#[test]
fn gfm_markdown_block_quotes_214() {
    // https://github.github.com/gfm/#example-214
    test_identical_markdown_events!(r##">     foo
    bar"##);
}

#[test]
fn gfm_markdown_block_quotes_215() {
    // https://github.github.com/gfm/#example-215
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
fn gfm_markdown_block_quotes_216() {
    // https://github.github.com/gfm/#example-216
    test_identical_markdown_events!(r##"> foo
    - bar"##,r##"> foo
> \- bar"##);
}

#[test]
fn gfm_markdown_block_quotes_217() {
    // https://github.github.com/gfm/#example-217
    test_identical_markdown_events!(r##">"##);
}

#[test]
fn gfm_markdown_block_quotes_218() {
    // https://github.github.com/gfm/#example-218
    test_identical_markdown_events!(">\n>  \n> ",r##">
>
>"##);
}

#[test]
fn gfm_markdown_block_quotes_219() {
    // https://github.github.com/gfm/#example-219
    test_identical_markdown_events!(">\n> foo\n>  ",r##"> foo
>"##);
}

#[test]
fn gfm_markdown_block_quotes_220() {
    // https://github.github.com/gfm/#example-220
    test_identical_markdown_events!(r##"> foo

> bar"##);
}

#[test]
fn gfm_markdown_block_quotes_221() {
    // https://github.github.com/gfm/#example-221
    test_identical_markdown_events!(r##"> foo
> bar"##);
}

#[test]
fn gfm_markdown_block_quotes_222() {
    // https://github.github.com/gfm/#example-222
    test_identical_markdown_events!(r##"> foo
>
> bar"##);
}

#[test]
fn gfm_markdown_block_quotes_223() {
    // https://github.github.com/gfm/#example-223
    test_identical_markdown_events!(r##"foo
> bar"##);
}

#[test]
fn gfm_markdown_block_quotes_224() {
    // https://github.github.com/gfm/#example-224
    test_identical_markdown_events!(r##"> aaa
***
> bbb"##);
}

#[test]
fn gfm_markdown_block_quotes_225() {
    // https://github.github.com/gfm/#example-225
    test_identical_markdown_events!(r##"> bar
baz"##,r##"> bar
> baz"##);
}

#[test]
fn gfm_markdown_block_quotes_226() {
    // https://github.github.com/gfm/#example-226
    test_identical_markdown_events!(r##"> bar

baz"##);
}

#[test]
fn gfm_markdown_block_quotes_227() {
    // https://github.github.com/gfm/#example-227
    test_identical_markdown_events!(r##"> bar
>
baz"##);
}

#[test]
fn gfm_markdown_block_quotes_228() {
    // https://github.github.com/gfm/#example-228
    test_identical_markdown_events!(r##"> > > foo
bar"##,r##"> > > foo
> > > bar"##);
}

#[test]
fn gfm_markdown_block_quotes_229() {
    // https://github.github.com/gfm/#example-229
    test_identical_markdown_events!(r##">>> foo
> bar
>>baz"##,r##"> > > foo
> > > bar
> > > baz"##);
}

#[test]
fn gfm_markdown_block_quotes_230() {
    // https://github.github.com/gfm/#example-230
    test_identical_markdown_events!(r##">     code

>    not code"##,r##">     code

> not code"##);
}

#[test]
fn gfm_markdown_list_items_231() {
    // https://github.github.com/gfm/#example-231
    test_identical_markdown_events!(r##"A paragraph
with two lines.

    indented code

> A block quote."##);
}

#[test]
fn gfm_markdown_list_items_232() {
    // https://github.github.com/gfm/#example-232
    test_identical_markdown_events!(r##"1.  A paragraph
    with two lines.

        indented code

    > A block quote."##,r##"1. A paragraph
   with two lines.

       indented code

   > A block quote."##);
}

#[test]
fn gfm_markdown_list_items_233() {
    // https://github.github.com/gfm/#example-233
    test_identical_markdown_events!(r##"- one

 two"##,r##"- one

two"##);
}

#[test]
fn gfm_markdown_list_items_234() {
    // https://github.github.com/gfm/#example-234
    test_identical_markdown_events!(r##"- one

  two"##);
}

#[test]
fn gfm_markdown_list_items_235() {
    // https://github.github.com/gfm/#example-235
    test!(r##" -    one

     two"##,r##"- one
<!-- Dont absorb code block into list -->
<!-- Consider a feenced code block instead -->

     two"##);
}

#[test]
fn gfm_markdown_list_items_236() {
    // https://github.github.com/gfm/#example-236
    test_identical_markdown_events!(r##" -    one

      two"##,r##"- one

  two"##);
}

#[test]
fn gfm_markdown_list_items_237() {
    // https://github.github.com/gfm/#example-237
    test_identical_markdown_events!(r##"   > > 1.  one
>>
>>     two"##,r##"> > 1. one
> >
> >    two"##);
}

#[test]
fn gfm_markdown_list_items_238() {
    // https://github.github.com/gfm/#example-238
    test_identical_markdown_events!(r##">>- one
>>
  >  > two"##,r##"> > - one
> >
> > two"##);
}

#[test]
fn gfm_markdown_list_items_239() {
    // https://github.github.com/gfm/#example-239
    test_identical_markdown_events!(r##"-one

2.two"##);
}

#[test]
fn gfm_markdown_list_items_240() {
    // https://github.github.com/gfm/#example-240
    test_identical_markdown_events!(r##"- foo


  bar"##);
}

#[test]
fn gfm_markdown_list_items_241() {
    // https://github.github.com/gfm/#example-241
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
fn gfm_markdown_list_items_242() {
    // https://github.github.com/gfm/#example-242
    test_identical_markdown_events!(r##"- Foo

      bar


      baz"##);
}

#[test]
fn gfm_markdown_list_items_243() {
    // https://github.github.com/gfm/#example-243
    test_identical_markdown_events!(r##"123456789. ok"##);
}

#[test]
fn gfm_markdown_list_items_244() {
    // https://github.github.com/gfm/#example-244
    test_identical_markdown_events!(r##"1234567890. not ok"##);
}

#[test]
fn gfm_markdown_list_items_245() {
    // https://github.github.com/gfm/#example-245
    test_identical_markdown_events!(r##"0. ok"##);
}

#[test]
fn gfm_markdown_list_items_246() {
    // https://github.github.com/gfm/#example-246
    test_identical_markdown_events!(r##"003. ok"##);
}

#[test]
fn gfm_markdown_list_items_247() {
    // https://github.github.com/gfm/#example-247
    test_identical_markdown_events!(r##"-1. not ok"##);
}

#[test]
fn gfm_markdown_list_items_248() {
    // https://github.github.com/gfm/#example-248
    test_identical_markdown_events!(r##"- foo

      bar"##);
}

#[test]
fn gfm_markdown_list_items_249() {
    // https://github.github.com/gfm/#example-249
    test_identical_markdown_events!(r##"  10.  foo

           bar"##,r##"10. foo

        bar"##);
}

#[test]
fn gfm_markdown_list_items_250() {
    // https://github.github.com/gfm/#example-250
    test_identical_markdown_events!(r##"    indented code

paragraph

    more code"##);
}

#[test]
fn gfm_markdown_list_items_251() {
    // https://github.github.com/gfm/#example-251
    test_identical_markdown_events!(r##"1.     indented code

   paragraph

       more code"##);
}

#[test]
fn gfm_markdown_list_items_252() {
    // https://github.github.com/gfm/#example-252
    test_identical_markdown_events!(r##"1.      indented code

   paragraph

       more code"##);
}

#[test]
fn gfm_markdown_list_items_253() {
    // https://github.github.com/gfm/#example-253
    test_identical_markdown_events!(r##"   foo

bar"##,r##"foo

bar"##);
}

#[test]
fn gfm_markdown_list_items_254() {
    // https://github.github.com/gfm/#example-254
    test_identical_markdown_events!(r##"-    foo

  bar"##,r##"- foo

bar"##);
}

#[test]
fn gfm_markdown_list_items_255() {
    // https://github.github.com/gfm/#example-255
    test_identical_markdown_events!(r##"-  foo

   bar"##,r##"- foo

  bar"##);
}

#[test]
fn gfm_markdown_list_items_256() {
    // https://github.github.com/gfm/#example-256
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
fn gfm_markdown_list_items_257() {
    // https://github.github.com/gfm/#example-257
    test_identical_markdown_events!("-   \n  foo",r##"- foo"##);
}

#[test]
fn gfm_markdown_list_items_258() {
    // https://github.github.com/gfm/#example-258
    test_identical_markdown_events!(r##"-

  foo"##,r##"-

foo"##);
}

#[test]
fn gfm_markdown_list_items_259() {
    // https://github.github.com/gfm/#example-259
    test_identical_markdown_events!(r##"- foo
-
- bar"##);
}

#[test]
fn gfm_markdown_list_items_260() {
    // https://github.github.com/gfm/#example-260
    test_identical_markdown_events!("- foo\n-   \n- bar",r##"- foo
-
- bar"##);
}

#[test]
fn gfm_markdown_list_items_261() {
    // https://github.github.com/gfm/#example-261
    test_identical_markdown_events!(r##"1. foo
2.
3. bar"##);
}

#[test]
fn gfm_markdown_list_items_262() {
    // https://github.github.com/gfm/#example-262
    test_identical_markdown_events!(r##"*"##);
}

#[test]
fn gfm_markdown_list_items_263() {
    // https://github.github.com/gfm/#example-263
    test_identical_markdown_events!(r##"foo
*

foo
1."##);
}

#[test]
fn gfm_markdown_list_items_264() {
    // https://github.github.com/gfm/#example-264
    test_identical_markdown_events!(r##" 1.  A paragraph
     with two lines.

         indented code

     > A block quote."##,r##"1. A paragraph
   with two lines.

       indented code

   > A block quote."##);
}

#[test]
fn gfm_markdown_list_items_265() {
    // https://github.github.com/gfm/#example-265
    test_identical_markdown_events!(r##"  1.  A paragraph
      with two lines.

          indented code

      > A block quote."##,r##"1. A paragraph
   with two lines.

       indented code

   > A block quote."##);
}

#[test]
fn gfm_markdown_list_items_266() {
    // https://github.github.com/gfm/#example-266
    test_identical_markdown_events!(r##"   1.  A paragraph
       with two lines.

           indented code

       > A block quote."##,r##"1. A paragraph
   with two lines.

       indented code

   > A block quote."##);
}

#[test]
fn gfm_markdown_list_items_267() {
    // https://github.github.com/gfm/#example-267
    test_identical_markdown_events!(r##"    1.  A paragraph
        with two lines.

            indented code

        > A block quote."##);
}

#[test]
fn gfm_markdown_list_items_268() {
    // https://github.github.com/gfm/#example-268
    test_identical_markdown_events!(r##"  1.  A paragraph
with two lines.

          indented code

      > A block quote."##,r##"1. A paragraph
   with two lines.

       indented code

   > A block quote."##);
}

#[test]
fn gfm_markdown_list_items_269() {
    // https://github.github.com/gfm/#example-269
    test_identical_markdown_events!(r##"  1.  A paragraph
    with two lines."##,r##"1. A paragraph
   with two lines."##);
}

#[test]
fn gfm_markdown_list_items_270() {
    // https://github.github.com/gfm/#example-270
    test_identical_markdown_events!(r##"> 1. > Blockquote
continued here."##,r##"> 1. > Blockquote
>    > continued here."##);
}

#[test]
fn gfm_markdown_list_items_271() {
    // https://github.github.com/gfm/#example-271
    test_identical_markdown_events!(r##"> 1. > Blockquote
> continued here."##,r##"> 1. > Blockquote
>    > continued here."##);
}

#[test]
fn gfm_markdown_list_items_272() {
    // https://github.github.com/gfm/#example-272
    test_identical_markdown_events!(r##"- foo
  - bar
    - baz
      - boo"##);
}

#[test]
fn gfm_markdown_list_items_273() {
    // https://github.github.com/gfm/#example-273
    test_identical_markdown_events!(r##"- foo
 - bar
  - baz
   - boo"##,r##"- foo
- bar
- baz
- boo"##);
}

#[test]
fn gfm_markdown_list_items_274() {
    // https://github.github.com/gfm/#example-274
    test_identical_markdown_events!(r##"10) foo
    - bar"##);
}

#[test]
fn gfm_markdown_list_items_275() {
    // https://github.github.com/gfm/#example-275
    test_identical_markdown_events!(r##"10) foo
   - bar"##,r##"10) foo
- bar"##);
}

#[test]
fn gfm_markdown_list_items_276() {
    // https://github.github.com/gfm/#example-276
    test_identical_markdown_events!(r##"- - foo"##);
}

#[test]
fn gfm_markdown_list_items_277() {
    // https://github.github.com/gfm/#example-277
    test_identical_markdown_events!(r##"1. - 2. foo"##);
}

#[test]
fn gfm_markdown_list_items_278() {
    // https://github.github.com/gfm/#example-278
    test_identical_markdown_events!(r##"- # Foo
- Bar
  ---
  baz"##);
}

#[test]
fn gfm_markdown_lists_281() {
    // https://github.github.com/gfm/#example-281
    test_identical_markdown_events!(r##"- foo
- bar
+ baz"##);
}

#[test]
fn gfm_markdown_lists_282() {
    // https://github.github.com/gfm/#example-282
    test_identical_markdown_events!(r##"1. foo
2. bar
3) baz"##);
}

#[test]
fn gfm_markdown_lists_283() {
    // https://github.github.com/gfm/#example-283
    test_identical_markdown_events!(r##"Foo
- bar
- baz"##);
}

#[test]
fn gfm_markdown_lists_284() {
    // https://github.github.com/gfm/#example-284
    test_identical_markdown_events!(r##"The number of windows in my house is
14.  The number of doors is 6."##);
}

#[test]
fn gfm_markdown_lists_285() {
    // https://github.github.com/gfm/#example-285
    test_identical_markdown_events!(r##"The number of windows in my house is
1.  The number of doors is 6."##,r##"The number of windows in my house is
1. The number of doors is 6."##);
}

#[test]
fn gfm_markdown_lists_286() {
    // https://github.github.com/gfm/#example-286
    test_identical_markdown_events!(r##"- foo

- bar


- baz"##);
}

#[test]
fn gfm_markdown_lists_287() {
    // https://github.github.com/gfm/#example-287
    test_identical_markdown_events!(r##"- foo
  - bar
    - baz


      bim"##);
}

#[test]
fn gfm_markdown_lists_288() {
    // https://github.github.com/gfm/#example-288
    test_identical_markdown_events!(r##"- foo
- bar

<!-- -->

- baz
- bim"##);
}

#[test]
fn gfm_markdown_lists_289() {
    // https://github.github.com/gfm/#example-289
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
fn gfm_markdown_lists_290() {
    // https://github.github.com/gfm/#example-290
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
fn gfm_markdown_lists_291() {
    // https://github.github.com/gfm/#example-291
    test_identical_markdown_events!(r##"1. a

  2. b

   3. c"##,r##"1. a

2. b

3. c"##);
}

#[test]
fn gfm_markdown_lists_292() {
    // https://github.github.com/gfm/#example-292
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
fn gfm_markdown_lists_293() {
    // https://github.github.com/gfm/#example-293
    test!(r##"1. a

  2. b

    3. c"##,r##"1. a

2. b
<!-- Dont absorb code block into list -->
<!-- Consider a feenced code block instead -->

    3. c"##);
}

#[test]
fn gfm_markdown_lists_294() {
    // https://github.github.com/gfm/#example-294
    test_identical_markdown_events!(r##"- a
- b

- c"##);
}

#[test]
fn gfm_markdown_lists_295() {
    // https://github.github.com/gfm/#example-295
    test_identical_markdown_events!(r##"* a
*

* c"##);
}

#[test]
fn gfm_markdown_lists_296() {
    // https://github.github.com/gfm/#example-296
    test_identical_markdown_events!(r##"- a
- b

  c
- d"##);
}

#[test]
fn gfm_markdown_lists_297() {
    // https://github.github.com/gfm/#example-297
    test!(r##"- a
- b

  [ref]: /url
- d"##,r##"- a
- b

[ref]: /url
- d"##);
}

#[test]
fn gfm_markdown_lists_298() {
    // https://github.github.com/gfm/#example-298
    test_identical_markdown_events!(r##"- a
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
fn gfm_markdown_lists_299() {
    // https://github.github.com/gfm/#example-299
    test_identical_markdown_events!(r##"- a
  - b

    c
- d"##);
}

#[test]
fn gfm_markdown_lists_300() {
    // https://github.github.com/gfm/#example-300
    test_identical_markdown_events!(r##"* a
  > b
  >
* c"##);
}

#[test]
fn gfm_markdown_lists_301() {
    // https://github.github.com/gfm/#example-301
    test_identical_markdown_events!(r##"- a
  > b
  ```
  c
  ```
- d"##);
}

#[test]
fn gfm_markdown_lists_302() {
    // https://github.github.com/gfm/#example-302
    test_identical_markdown_events!(r##"- a"##);
}

#[test]
fn gfm_markdown_lists_303() {
    // https://github.github.com/gfm/#example-303
    test_identical_markdown_events!(r##"- a
  - b"##);
}

#[test]
fn gfm_markdown_lists_304() {
    // https://github.github.com/gfm/#example-304
    test_identical_markdown_events!(r##"1. ```
   foo
   ```

   bar"##);
}

#[test]
fn gfm_markdown_lists_305() {
    // https://github.github.com/gfm/#example-305
    test_identical_markdown_events!(r##"* foo
  * bar

  baz"##);
}

#[test]
fn gfm_markdown_lists_306() {
    // https://github.github.com/gfm/#example-306
    test_identical_markdown_events!(r##"- a
  - b
  - c

- d
  - e
  - f"##);
}

#[test]
fn gfm_markdown_inlines_307() {
    // https://github.github.com/gfm/#example-307
    test_identical_markdown_events!(r##"`hi`lo`"##);
}

#[test]
fn gfm_markdown_backslash_escapes_308() {
    // https://github.github.com/gfm/#example-308
    test_identical_markdown_events!(r##"\!\"\#\$\%\&\'\(\)\*\+\,\-\.\/\:\;\<\=\>\?\@\[\\\]\^\_\`\{\|\}\~"##);
}

#[test]
fn gfm_markdown_backslash_escapes_309() {
    // https://github.github.com/gfm/#example-309
    test_identical_markdown_events!(r##"\	\A\a\ \3\φ\«"##);
}

#[test]
fn gfm_markdown_backslash_escapes_310() {
    // https://github.github.com/gfm/#example-310
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
fn gfm_markdown_backslash_escapes_311() {
    // https://github.github.com/gfm/#example-311
    test_identical_markdown_events!(r##"\\*emphasis*"##);
}

#[test]
fn gfm_markdown_backslash_escapes_312() {
    // https://github.github.com/gfm/#example-312
    test_identical_markdown_events!(r##"foo\
bar"##);
}

#[test]
fn gfm_markdown_backslash_escapes_313() {
    // https://github.github.com/gfm/#example-313
    test_identical_markdown_events!(r##"`` \[\` ``"##);
}

#[test]
fn gfm_markdown_backslash_escapes_314() {
    // https://github.github.com/gfm/#example-314
    test_identical_markdown_events!(r##"    \[\]"##);
}

#[test]
fn gfm_markdown_backslash_escapes_315() {
    // https://github.github.com/gfm/#example-315
    test_identical_markdown_events!(r##"~~~
\[\]
~~~"##);
}

#[test]
fn gfm_markdown_backslash_escapes_316() {
    // https://github.github.com/gfm/#example-316
    test_identical_markdown_events!(r##"<http://example.com?find=\*>"##);
}

#[test]
fn gfm_markdown_backslash_escapes_317() {
    // https://github.github.com/gfm/#example-317
    test_identical_markdown_events!(r##"<a href="/bar\/)">"##);
}

#[test]
fn gfm_markdown_backslash_escapes_318() {
    // https://github.github.com/gfm/#example-318
    test_identical_markdown_events!(r##"[foo](/bar\* "ti\*tle")"##);
}

#[test]
fn gfm_markdown_backslash_escapes_319() {
    // https://github.github.com/gfm/#example-319
    test_identical_markdown_events!(r##"[foo]

[foo]: /bar\* "ti\*tle""##);
}

#[test]
fn gfm_markdown_backslash_escapes_320() {
    // https://github.github.com/gfm/#example-320
    test_identical_markdown_events!(r##"``` foo\+bar
foo
```"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_321() {
    // https://github.github.com/gfm/#example-321
    test_identical_markdown_events!(r##"&nbsp; &amp; &copy; &AElig; &Dcaron;
&frac34; &HilbertSpace; &DifferentialD;
&ClockwiseContourIntegral; &ngE;"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_322() {
    // https://github.github.com/gfm/#example-322
    test_identical_markdown_events!(r##"&#35; &#1234; &#992; &#0;"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_323() {
    // https://github.github.com/gfm/#example-323
    test_identical_markdown_events!(r##"&#X22; &#XD06; &#xcab;"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_324() {
    // https://github.github.com/gfm/#example-324
    test_identical_markdown_events!(r##"&nbsp &x; &#; &#x;
&#987654321;
&#abcdef0;
&ThisIsNotDefined; &hi?;"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_325() {
    // https://github.github.com/gfm/#example-325
    test_identical_markdown_events!(r##"&copy"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_326() {
    // https://github.github.com/gfm/#example-326
    test_identical_markdown_events!(r##"&MadeUpEntity;"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_327() {
    // https://github.github.com/gfm/#example-327
    test_identical_markdown_events!(r##"<a href="&ouml;&ouml;.html">"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_328() {
    // https://github.github.com/gfm/#example-328
    test_identical_markdown_events!(r##"[foo](/f&ouml;&ouml; "f&ouml;&ouml;")"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_329() {
    // https://github.github.com/gfm/#example-329
    test_identical_markdown_events!(r##"[foo]

[foo]: /f&ouml;&ouml; "f&ouml;&ouml;""##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_330() {
    // https://github.github.com/gfm/#example-330
    test_identical_markdown_events!(r##"``` f&ouml;&ouml;
foo
```"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_331() {
    // https://github.github.com/gfm/#example-331
    test_identical_markdown_events!(r##"`f&ouml;&ouml;`"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_332() {
    // https://github.github.com/gfm/#example-332
    test_identical_markdown_events!(r##"    f&ouml;f&ouml;"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_333() {
    // https://github.github.com/gfm/#example-333
    test_identical_markdown_events!(r##"&#42;foo&#42;
*foo*"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_334() {
    // https://github.github.com/gfm/#example-334
    test_identical_markdown_events!(r##"&#42; foo

* foo"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_335() {
    // https://github.github.com/gfm/#example-335
    test_identical_markdown_events!(r##"foo&#10;&#10;bar"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_336() {
    // https://github.github.com/gfm/#example-336
    test_identical_markdown_events!(r##"&#9;foo"##);
}

#[test]
fn gfm_markdown_entity_and_numeric_character_references_337() {
    // https://github.github.com/gfm/#example-337
    test_identical_markdown_events!(r##"[a](url &quot;tit&quot;)"##);
}

#[test]
fn gfm_markdown_code_spans_338() {
    // https://github.github.com/gfm/#example-338
    test_identical_markdown_events!(r##"`foo`"##);
}

#[test]
fn gfm_markdown_code_spans_339() {
    // https://github.github.com/gfm/#example-339
    test_identical_markdown_events!(r##"`` foo ` bar ``"##);
}

#[test]
fn gfm_markdown_code_spans_340() {
    // https://github.github.com/gfm/#example-340
    test_identical_markdown_events!(r##"` `` `"##);
}

#[test]
fn gfm_markdown_code_spans_341() {
    // https://github.github.com/gfm/#example-341
    test_identical_markdown_events!(r##"`  ``  `"##);
}

#[test]
fn gfm_markdown_code_spans_342() {
    // https://github.github.com/gfm/#example-342
    test_identical_markdown_events!(r##"` a`"##);
}

#[test]
fn gfm_markdown_code_spans_343() {
    // https://github.github.com/gfm/#example-343
    test_identical_markdown_events!(r##"` b `"##);
}

#[test]
fn gfm_markdown_code_spans_344() {
    // https://github.github.com/gfm/#example-344
    test_identical_markdown_events!(r##"` `
`  `"##);
}

#[test]
fn gfm_markdown_code_spans_345() {
    // https://github.github.com/gfm/#example-345
    test_identical_markdown_events!("``\nfoo\nbar  \nbaz\n``");
}

#[test]
fn gfm_markdown_code_spans_346() {
    // https://github.github.com/gfm/#example-346
    test_identical_markdown_events!("``\nfoo \n``");
}

#[test]
fn gfm_markdown_code_spans_347() {
    // https://github.github.com/gfm/#example-347
    test_identical_markdown_events!("`foo   bar \nbaz`");
}

#[test]
fn gfm_markdown_code_spans_348() {
    // https://github.github.com/gfm/#example-348
    test_identical_markdown_events!(r##"`foo\`bar`"##);
}

#[test]
fn gfm_markdown_code_spans_349() {
    // https://github.github.com/gfm/#example-349
    test_identical_markdown_events!(r##"``foo`bar``"##);
}

#[test]
fn gfm_markdown_code_spans_350() {
    // https://github.github.com/gfm/#example-350
    test_identical_markdown_events!(r##"` foo `` bar `"##);
}

#[test]
fn gfm_markdown_code_spans_351() {
    // https://github.github.com/gfm/#example-351
    test_identical_markdown_events!(r##"*foo`*`"##);
}

#[test]
fn gfm_markdown_code_spans_352() {
    // https://github.github.com/gfm/#example-352
    test_identical_markdown_events!(r##"[not a `link](/foo`)"##);
}

#[test]
fn gfm_markdown_code_spans_353() {
    // https://github.github.com/gfm/#example-353
    test_identical_markdown_events!(r##"`<a href="`">`"##);
}

#[test]
fn gfm_markdown_code_spans_354() {
    // https://github.github.com/gfm/#example-354
    test_identical_markdown_events!(r##"<a href="`">`"##);
}

#[test]
fn gfm_markdown_code_spans_355() {
    // https://github.github.com/gfm/#example-355
    test_identical_markdown_events!(r##"`<http://foo.bar.`baz>`"##);
}

#[test]
fn gfm_markdown_code_spans_356() {
    // https://github.github.com/gfm/#example-356
    test_identical_markdown_events!(r##"<http://foo.bar.`baz>`"##);
}

#[test]
fn gfm_markdown_code_spans_357() {
    // https://github.github.com/gfm/#example-357
    test_identical_markdown_events!(r##"```foo``"##);
}

#[test]
fn gfm_markdown_code_spans_358() {
    // https://github.github.com/gfm/#example-358
    test_identical_markdown_events!(r##"`foo"##);
}

#[test]
fn gfm_markdown_code_spans_359() {
    // https://github.github.com/gfm/#example-359
    test_identical_markdown_events!(r##"`foo``bar``"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_360() {
    // https://github.github.com/gfm/#example-360
    test_identical_markdown_events!(r##"*foo bar*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_361() {
    // https://github.github.com/gfm/#example-361
    test_identical_markdown_events!(r##"a * foo bar*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_362() {
    // https://github.github.com/gfm/#example-362
    test_identical_markdown_events!(r##"a*"foo"*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_363() {
    // https://github.github.com/gfm/#example-363
    test_identical_markdown_events!(r##"* a *"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_364() {
    // https://github.github.com/gfm/#example-364
    test_identical_markdown_events!(r##"foo*bar*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_365() {
    // https://github.github.com/gfm/#example-365
    test_identical_markdown_events!(r##"5*6*78"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_366() {
    // https://github.github.com/gfm/#example-366
    test_identical_markdown_events!(r##"_foo bar_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_367() {
    // https://github.github.com/gfm/#example-367
    test_identical_markdown_events!(r##"_ foo bar_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_368() {
    // https://github.github.com/gfm/#example-368
    test_identical_markdown_events!(r##"a_"foo"_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_369() {
    // https://github.github.com/gfm/#example-369
    test_identical_markdown_events!(r##"foo_bar_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_370() {
    // https://github.github.com/gfm/#example-370
    test_identical_markdown_events!(r##"5_6_78"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_371() {
    // https://github.github.com/gfm/#example-371
    test_identical_markdown_events!(r##"пристаням_стремятся_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_372() {
    // https://github.github.com/gfm/#example-372
    test_identical_markdown_events!(r##"aa_"bb"_cc"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_373() {
    // https://github.github.com/gfm/#example-373
    test_identical_markdown_events!(r##"foo-_(bar)_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_374() {
    // https://github.github.com/gfm/#example-374
    test_identical_markdown_events!(r##"_foo*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_375() {
    // https://github.github.com/gfm/#example-375
    test_identical_markdown_events!(r##"*foo bar *"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_376() {
    // https://github.github.com/gfm/#example-376
    test_identical_markdown_events!(r##"*foo bar
*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_377() {
    // https://github.github.com/gfm/#example-377
    test_identical_markdown_events!(r##"*(*foo)"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_378() {
    // https://github.github.com/gfm/#example-378
    test_identical_markdown_events!(r##"*(*foo*)*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_379() {
    // https://github.github.com/gfm/#example-379
    test_identical_markdown_events!(r##"*foo*bar"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_380() {
    // https://github.github.com/gfm/#example-380
    test_identical_markdown_events!(r##"_foo bar _"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_381() {
    // https://github.github.com/gfm/#example-381
    test_identical_markdown_events!(r##"_(_foo)"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_382() {
    // https://github.github.com/gfm/#example-382
    test_identical_markdown_events!(r##"_(_foo_)_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_383() {
    // https://github.github.com/gfm/#example-383
    test_identical_markdown_events!(r##"_foo_bar"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_384() {
    // https://github.github.com/gfm/#example-384
    test_identical_markdown_events!(r##"_пристаням_стремятся"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_385() {
    // https://github.github.com/gfm/#example-385
    test_identical_markdown_events!(r##"_foo_bar_baz_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_386() {
    // https://github.github.com/gfm/#example-386
    test_identical_markdown_events!(r##"_(bar)_."##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_387() {
    // https://github.github.com/gfm/#example-387
    test_identical_markdown_events!(r##"**foo bar**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_388() {
    // https://github.github.com/gfm/#example-388
    test_identical_markdown_events!(r##"** foo bar**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_389() {
    // https://github.github.com/gfm/#example-389
    test_identical_markdown_events!(r##"a**"foo"**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_390() {
    // https://github.github.com/gfm/#example-390
    test_identical_markdown_events!(r##"foo**bar**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_391() {
    // https://github.github.com/gfm/#example-391
    test_identical_markdown_events!(r##"__foo bar__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_392() {
    // https://github.github.com/gfm/#example-392
    test_identical_markdown_events!(r##"__ foo bar__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_393() {
    // https://github.github.com/gfm/#example-393
    test_identical_markdown_events!(r##"__
foo bar__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_394() {
    // https://github.github.com/gfm/#example-394
    test_identical_markdown_events!(r##"a__"foo"__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_395() {
    // https://github.github.com/gfm/#example-395
    test_identical_markdown_events!(r##"foo__bar__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_396() {
    // https://github.github.com/gfm/#example-396
    test_identical_markdown_events!(r##"5__6__78"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_397() {
    // https://github.github.com/gfm/#example-397
    test_identical_markdown_events!(r##"пристаням__стремятся__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_398() {
    // https://github.github.com/gfm/#example-398
    test_identical_markdown_events!(r##"__foo, __bar__, baz__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_399() {
    // https://github.github.com/gfm/#example-399
    test_identical_markdown_events!(r##"foo-__(bar)__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_400() {
    // https://github.github.com/gfm/#example-400
    test_identical_markdown_events!(r##"**foo bar **"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_401() {
    // https://github.github.com/gfm/#example-401
    test_identical_markdown_events!(r##"**(**foo)"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_402() {
    // https://github.github.com/gfm/#example-402
    test_identical_markdown_events!(r##"*(**foo**)*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_403() {
    // https://github.github.com/gfm/#example-403
    test_identical_markdown_events!(r##"**Gomphocarpus (*Gomphocarpus physocarpus*, syn.
*Asclepias physocarpa*)**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_404() {
    // https://github.github.com/gfm/#example-404
    test_identical_markdown_events!(r##"**foo "*bar*" foo**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_405() {
    // https://github.github.com/gfm/#example-405
    test_identical_markdown_events!(r##"**foo**bar"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_406() {
    // https://github.github.com/gfm/#example-406
    test_identical_markdown_events!(r##"__foo bar __"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_407() {
    // https://github.github.com/gfm/#example-407
    test_identical_markdown_events!(r##"__(__foo)"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_408() {
    // https://github.github.com/gfm/#example-408
    test_identical_markdown_events!(r##"_(__foo__)_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_409() {
    // https://github.github.com/gfm/#example-409
    test_identical_markdown_events!(r##"__foo__bar"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_410() {
    // https://github.github.com/gfm/#example-410
    test_identical_markdown_events!(r##"__пристаням__стремятся"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_411() {
    // https://github.github.com/gfm/#example-411
    test_identical_markdown_events!(r##"__foo__bar__baz__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_412() {
    // https://github.github.com/gfm/#example-412
    test_identical_markdown_events!(r##"__(bar)__."##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_413() {
    // https://github.github.com/gfm/#example-413
    test_identical_markdown_events!(r##"*foo [bar](/url)*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_414() {
    // https://github.github.com/gfm/#example-414
    test_identical_markdown_events!(r##"*foo
bar*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_415() {
    // https://github.github.com/gfm/#example-415
    test_identical_markdown_events!(r##"_foo __bar__ baz_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_416() {
    // https://github.github.com/gfm/#example-416
    test_identical_markdown_events!(r##"_foo _bar_ baz_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_417() {
    // https://github.github.com/gfm/#example-417
    test_identical_markdown_events!(r##"__foo_ bar_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_418() {
    // https://github.github.com/gfm/#example-418
    test_identical_markdown_events!(r##"*foo *bar**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_419() {
    // https://github.github.com/gfm/#example-419
    test_identical_markdown_events!(r##"*foo **bar** baz*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_420() {
    // https://github.github.com/gfm/#example-420
    test_identical_markdown_events!(r##"*foo**bar**baz*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_421() {
    // https://github.github.com/gfm/#example-421
    test_identical_markdown_events!(r##"*foo**bar*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_422() {
    // https://github.github.com/gfm/#example-422
    test_identical_markdown_events!(r##"***foo** bar*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_423() {
    // https://github.github.com/gfm/#example-423
    test_identical_markdown_events!(r##"*foo **bar***"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_424() {
    // https://github.github.com/gfm/#example-424
    test_identical_markdown_events!(r##"*foo**bar***"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_425() {
    // https://github.github.com/gfm/#example-425
    test_identical_markdown_events!(r##"foo***bar***baz"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_426() {
    // https://github.github.com/gfm/#example-426
    test_identical_markdown_events!(r##"foo******bar*********baz"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_427() {
    // https://github.github.com/gfm/#example-427
    test_identical_markdown_events!(r##"*foo **bar *baz* bim** bop*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_428() {
    // https://github.github.com/gfm/#example-428
    test_identical_markdown_events!(r##"*foo [*bar*](/url)*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_429() {
    // https://github.github.com/gfm/#example-429
    test_identical_markdown_events!(r##"** is not an empty emphasis"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_430() {
    // https://github.github.com/gfm/#example-430
    test_identical_markdown_events!(r##"**** is not an empty strong emphasis"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_431() {
    // https://github.github.com/gfm/#example-431
    test_identical_markdown_events!(r##"**foo [bar](/url)**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_432() {
    // https://github.github.com/gfm/#example-432
    test_identical_markdown_events!(r##"**foo
bar**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_433() {
    // https://github.github.com/gfm/#example-433
    test_identical_markdown_events!(r##"__foo _bar_ baz__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_434() {
    // https://github.github.com/gfm/#example-434
    test_identical_markdown_events!(r##"__foo __bar__ baz__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_435() {
    // https://github.github.com/gfm/#example-435
    test_identical_markdown_events!(r##"____foo__ bar__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_436() {
    // https://github.github.com/gfm/#example-436
    test_identical_markdown_events!(r##"**foo **bar****"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_437() {
    // https://github.github.com/gfm/#example-437
    test_identical_markdown_events!(r##"**foo *bar* baz**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_438() {
    // https://github.github.com/gfm/#example-438
    test_identical_markdown_events!(r##"**foo*bar*baz**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_439() {
    // https://github.github.com/gfm/#example-439
    test_identical_markdown_events!(r##"***foo* bar**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_440() {
    // https://github.github.com/gfm/#example-440
    test_identical_markdown_events!(r##"**foo *bar***"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_441() {
    // https://github.github.com/gfm/#example-441
    test_identical_markdown_events!(r##"**foo *bar **baz**
bim* bop**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_442() {
    // https://github.github.com/gfm/#example-442
    test_identical_markdown_events!(r##"**foo [*bar*](/url)**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_443() {
    // https://github.github.com/gfm/#example-443
    test_identical_markdown_events!(r##"__ is not an empty emphasis"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_444() {
    // https://github.github.com/gfm/#example-444
    test_identical_markdown_events!(r##"____ is not an empty strong emphasis"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_445() {
    // https://github.github.com/gfm/#example-445
    test_identical_markdown_events!(r##"foo ***"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_446() {
    // https://github.github.com/gfm/#example-446
    test_identical_markdown_events!(r##"foo *\**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_447() {
    // https://github.github.com/gfm/#example-447
    test_identical_markdown_events!(r##"foo *_*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_448() {
    // https://github.github.com/gfm/#example-448
    test_identical_markdown_events!(r##"foo *****"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_449() {
    // https://github.github.com/gfm/#example-449
    test_identical_markdown_events!(r##"foo **\***"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_450() {
    // https://github.github.com/gfm/#example-450
    test_identical_markdown_events!(r##"foo **_**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_451() {
    // https://github.github.com/gfm/#example-451
    test_identical_markdown_events!(r##"**foo*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_452() {
    // https://github.github.com/gfm/#example-452
    test_identical_markdown_events!(r##"*foo**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_453() {
    // https://github.github.com/gfm/#example-453
    test_identical_markdown_events!(r##"***foo**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_454() {
    // https://github.github.com/gfm/#example-454
    test_identical_markdown_events!(r##"****foo*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_455() {
    // https://github.github.com/gfm/#example-455
    test_identical_markdown_events!(r##"**foo***"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_456() {
    // https://github.github.com/gfm/#example-456
    test_identical_markdown_events!(r##"*foo****"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_457() {
    // https://github.github.com/gfm/#example-457
    test_identical_markdown_events!(r##"foo ___"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_458() {
    // https://github.github.com/gfm/#example-458
    test_identical_markdown_events!(r##"foo _\__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_459() {
    // https://github.github.com/gfm/#example-459
    test_identical_markdown_events!(r##"foo _*_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_460() {
    // https://github.github.com/gfm/#example-460
    test_identical_markdown_events!(r##"foo _____"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_461() {
    // https://github.github.com/gfm/#example-461
    test_identical_markdown_events!(r##"foo __\___"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_462() {
    // https://github.github.com/gfm/#example-462
    test_identical_markdown_events!(r##"foo __*__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_463() {
    // https://github.github.com/gfm/#example-463
    test_identical_markdown_events!(r##"__foo_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_464() {
    // https://github.github.com/gfm/#example-464
    test_identical_markdown_events!(r##"_foo__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_465() {
    // https://github.github.com/gfm/#example-465
    test_identical_markdown_events!(r##"___foo__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_466() {
    // https://github.github.com/gfm/#example-466
    test_identical_markdown_events!(r##"____foo_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_467() {
    // https://github.github.com/gfm/#example-467
    test_identical_markdown_events!(r##"__foo___"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_468() {
    // https://github.github.com/gfm/#example-468
    test_identical_markdown_events!(r##"_foo____"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_469() {
    // https://github.github.com/gfm/#example-469
    test_identical_markdown_events!(r##"**foo**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_470() {
    // https://github.github.com/gfm/#example-470
    test_identical_markdown_events!(r##"*_foo_*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_471() {
    // https://github.github.com/gfm/#example-471
    test_identical_markdown_events!(r##"__foo__"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_472() {
    // https://github.github.com/gfm/#example-472
    test_identical_markdown_events!(r##"_*foo*_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_473() {
    // https://github.github.com/gfm/#example-473
    test_identical_markdown_events!(r##"****foo****"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_474() {
    // https://github.github.com/gfm/#example-474
    test_identical_markdown_events!(r##"____foo____"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_475() {
    // https://github.github.com/gfm/#example-475
    test_identical_markdown_events!(r##"******foo******"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_476() {
    // https://github.github.com/gfm/#example-476
    test_identical_markdown_events!(r##"***foo***"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_477() {
    // https://github.github.com/gfm/#example-477
    test_identical_markdown_events!(r##"_____foo_____"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_478() {
    // https://github.github.com/gfm/#example-478
    test_identical_markdown_events!(r##"*foo _bar* baz_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_479() {
    // https://github.github.com/gfm/#example-479
    test_identical_markdown_events!(r##"*foo __bar *baz bim__ bam*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_480() {
    // https://github.github.com/gfm/#example-480
    test_identical_markdown_events!(r##"**foo **bar baz**"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_481() {
    // https://github.github.com/gfm/#example-481
    test_identical_markdown_events!(r##"*foo *bar baz*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_482() {
    // https://github.github.com/gfm/#example-482
    test_identical_markdown_events!(r##"*[bar*](/url)"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_483() {
    // https://github.github.com/gfm/#example-483
    test_identical_markdown_events!(r##"_foo [bar_](/url)"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_484() {
    // https://github.github.com/gfm/#example-484
    test_identical_markdown_events!(r##"*<img src="foo" title="*"/>"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_485() {
    // https://github.github.com/gfm/#example-485
    test_identical_markdown_events!(r##"**<a href="**">"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_486() {
    // https://github.github.com/gfm/#example-486
    test_identical_markdown_events!(r##"__<a href="__">"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_487() {
    // https://github.github.com/gfm/#example-487
    test_identical_markdown_events!(r##"*a `*`*"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_488() {
    // https://github.github.com/gfm/#example-488
    test_identical_markdown_events!(r##"_a `_`_"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_489() {
    // https://github.github.com/gfm/#example-489
    test_identical_markdown_events!(r##"**a<http://foo.bar/?q=**>"##);
}

#[test]
fn gfm_markdown_emphasis_and_strong_emphasis_490() {
    // https://github.github.com/gfm/#example-490
    test_identical_markdown_events!(r##"__a<http://foo.bar/?q=__>"##);
}

#[test]
fn gfm_markdown_strikethrough_extension_491() {
    // https://github.github.com/gfm/#example-491
    test_identical_markdown_events!(r##"~~Hi~~ Hello, world!"##);
}

#[test]
fn gfm_markdown_strikethrough_extension_492() {
    // https://github.github.com/gfm/#example-492
    test_identical_markdown_events!(r##"This ~~has a

new paragraph~~."##);
}

#[test]
fn gfm_markdown_links_493() {
    // https://github.github.com/gfm/#example-493
    test_identical_markdown_events!(r##"[link](/uri "title")"##);
}

#[test]
fn gfm_markdown_links_494() {
    // https://github.github.com/gfm/#example-494
    test_identical_markdown_events!(r##"[link](/uri)"##);
}

#[test]
fn gfm_markdown_links_495() {
    // https://github.github.com/gfm/#example-495
    test_identical_markdown_events!(r##"[link]()"##);
}

#[test]
fn gfm_markdown_links_496() {
    // https://github.github.com/gfm/#example-496
    test_identical_markdown_events!(r##"[link](<>)"##,r##"[link]()"##);
}

#[test]
fn gfm_markdown_links_497() {
    // https://github.github.com/gfm/#example-497
    test_identical_markdown_events!(r##"[link](/my uri)"##);
}

#[test]
fn gfm_markdown_links_498() {
    // https://github.github.com/gfm/#example-498
    test_identical_markdown_events!(r##"[link](</my uri>)"##);
}

#[test]
fn gfm_markdown_links_499() {
    // https://github.github.com/gfm/#example-499
    test_identical_markdown_events!(r##"[link](foo
bar)"##);
}

#[test]
fn gfm_markdown_links_500() {
    // https://github.github.com/gfm/#example-500
    test_identical_markdown_events!(r##"[link](<foo
bar>)"##);
}

#[test]
fn gfm_markdown_links_501() {
    // https://github.github.com/gfm/#example-501
    test_identical_markdown_events!(r##"[a](<b)c>)"##);
}

#[test]
fn gfm_markdown_links_502() {
    // https://github.github.com/gfm/#example-502
    test_identical_markdown_events!(r##"[link](<foo\>)"##);
}

#[test]
fn gfm_markdown_links_503() {
    // https://github.github.com/gfm/#example-503
    test_identical_markdown_events!(r##"[a](<b)c
[a](<b)c>
[a](<b>c)"##);
}

#[test]
fn gfm_markdown_links_504() {
    // https://github.github.com/gfm/#example-504
    test_identical_markdown_events!(r##"[link](\(foo\))"##);
}

#[test]
fn gfm_markdown_links_505() {
    // https://github.github.com/gfm/#example-505
    test_identical_markdown_events!(r##"[link](foo(and(bar)))"##);
}

#[test]
fn gfm_markdown_links_506() {
    // https://github.github.com/gfm/#example-506
    test_identical_markdown_events!(r##"[link](foo\(and\(bar\))"##);
}

#[test]
fn gfm_markdown_links_507() {
    // https://github.github.com/gfm/#example-507
    test_identical_markdown_events!(r##"[link](<foo(and(bar)>)"##);
}

#[test]
fn gfm_markdown_links_508() {
    // https://github.github.com/gfm/#example-508
    test_identical_markdown_events!(r##"[link](foo\)\:)"##);
}

#[test]
fn gfm_markdown_links_509() {
    // https://github.github.com/gfm/#example-509
    test_identical_markdown_events!(r##"[link](#fragment)

[link](http://example.com#fragment)

[link](http://example.com?foo=3#frag)"##);
}

#[test]
fn gfm_markdown_links_510() {
    // https://github.github.com/gfm/#example-510
    test_identical_markdown_events!(r##"[link](foo\bar)"##);
}

#[test]
fn gfm_markdown_links_511() {
    // https://github.github.com/gfm/#example-511
    test_identical_markdown_events!(r##"[link](foo%20b&auml;)"##);
}

#[test]
fn gfm_markdown_links_512() {
    // https://github.github.com/gfm/#example-512
    test_identical_markdown_events!(r##"[link]("title")"##);
}

#[test]
fn gfm_markdown_links_513() {
    // https://github.github.com/gfm/#example-513
    test_identical_markdown_events!(r##"[link](/url "title")
[link](/url 'title')
[link](/url (title))"##);
}

#[test]
fn gfm_markdown_links_514() {
    // https://github.github.com/gfm/#example-514
    test_identical_markdown_events!(r##"[link](/url "title \"&quot;")"##);
}

#[test]
fn gfm_markdown_links_515() {
    // https://github.github.com/gfm/#example-515
    test!(r##"[link](/url "title")"##,r##"[link](/url "title")"##);
}

#[test]
fn gfm_markdown_links_516() {
    // https://github.github.com/gfm/#example-516
    test_identical_markdown_events!(r##"[link](/url "title "and" title")"##);
}

#[test]
fn gfm_markdown_links_517() {
    // https://github.github.com/gfm/#example-517
    test_identical_markdown_events!(r##"[link](/url 'title "and" title')"##);
}

#[test]
fn gfm_markdown_links_518() {
    // https://github.github.com/gfm/#example-518
    test_identical_markdown_events!(r##"[link](   /uri
  "title"  )"##,r##"[link](/uri "title")"##);
}

#[test]
fn gfm_markdown_links_519() {
    // https://github.github.com/gfm/#example-519
    test_identical_markdown_events!(r##"[link] (/uri)"##);
}

#[test]
fn gfm_markdown_links_520() {
    // https://github.github.com/gfm/#example-520
    test_identical_markdown_events!(r##"[link [foo [bar]]](/uri)"##);
}

#[test]
fn gfm_markdown_links_521() {
    // https://github.github.com/gfm/#example-521
    test_identical_markdown_events!(r##"[link] bar](/uri)"##);
}

#[test]
fn gfm_markdown_links_522() {
    // https://github.github.com/gfm/#example-522
    test_identical_markdown_events!(r##"[link [bar](/uri)"##);
}

#[test]
fn gfm_markdown_links_523() {
    // https://github.github.com/gfm/#example-523
    test_identical_markdown_events!(r##"[link \[bar](/uri)"##);
}

#[test]
fn gfm_markdown_links_524() {
    // https://github.github.com/gfm/#example-524
    test_identical_markdown_events!(r##"[link *foo **bar** `#`*](/uri)"##);
}

#[test]
fn gfm_markdown_links_525() {
    // https://github.github.com/gfm/#example-525
    test_identical_markdown_events!(r##"[![moon](moon.jpg)](/uri)"##);
}

#[test]
fn gfm_markdown_links_526() {
    // https://github.github.com/gfm/#example-526
    test_identical_markdown_events!(r##"[foo [bar](/uri)](/uri)"##);
}

#[test]
fn gfm_markdown_links_527() {
    // https://github.github.com/gfm/#example-527
    test_identical_markdown_events!(r##"[foo *[bar [baz](/uri)](/uri)*](/uri)"##);
}

#[test]
fn gfm_markdown_links_528() {
    // https://github.github.com/gfm/#example-528
    test_identical_markdown_events!(r##"![[[foo](uri1)](uri2)](uri3)"##);
}

#[test]
fn gfm_markdown_links_529() {
    // https://github.github.com/gfm/#example-529
    test_identical_markdown_events!(r##"*[foo*](/uri)"##);
}

#[test]
fn gfm_markdown_links_530() {
    // https://github.github.com/gfm/#example-530
    test_identical_markdown_events!(r##"[foo *bar](baz*)"##);
}

#[test]
fn gfm_markdown_links_531() {
    // https://github.github.com/gfm/#example-531
    test_identical_markdown_events!(r##"*foo [bar* baz]"##);
}

#[test]
fn gfm_markdown_links_532() {
    // https://github.github.com/gfm/#example-532
    test_identical_markdown_events!(r##"[foo <bar attr="](baz)">"##);
}

#[test]
fn gfm_markdown_links_533() {
    // https://github.github.com/gfm/#example-533
    test_identical_markdown_events!(r##"[foo`](/uri)`"##);
}

#[test]
fn gfm_markdown_links_534() {
    // https://github.github.com/gfm/#example-534
    test_identical_markdown_events!(r##"[foo<http://example.com/?search=](uri)>"##);
}

#[test]
fn gfm_markdown_links_535() {
    // https://github.github.com/gfm/#example-535
    test_identical_markdown_events!(r##"[foo][bar]

[bar]: /url "title""##);
}

#[test]
fn gfm_markdown_links_536() {
    // https://github.github.com/gfm/#example-536
    test_identical_markdown_events!(r##"[link [foo [bar]]][ref]

[ref]: /uri"##);
}

#[test]
fn gfm_markdown_links_537() {
    // https://github.github.com/gfm/#example-537
    test_identical_markdown_events!(r##"[link \[bar][ref]

[ref]: /uri"##);
}

#[test]
fn gfm_markdown_links_538() {
    // https://github.github.com/gfm/#example-538
    test_identical_markdown_events!(r##"[link *foo **bar** `#`*][ref]

[ref]: /uri"##);
}

#[test]
fn gfm_markdown_links_539() {
    // https://github.github.com/gfm/#example-539
    test_identical_markdown_events!(r##"[![moon](moon.jpg)][ref]

[ref]: /uri"##);
}

#[test]
fn gfm_markdown_links_540() {
    // https://github.github.com/gfm/#example-540
    test_identical_markdown_events!(r##"[foo [bar](/uri)][ref]

[ref]: /uri"##);
}

#[test]
fn gfm_markdown_links_541() {
    // https://github.github.com/gfm/#example-541
    test_identical_markdown_events!(r##"[foo *bar [baz][ref]*][ref]

[ref]: /uri"##);
}

#[test]
fn gfm_markdown_links_542() {
    // https://github.github.com/gfm/#example-542
    test_identical_markdown_events!(r##"*[foo*][ref]

[ref]: /uri"##);
}

#[test]
fn gfm_markdown_links_543() {
    // https://github.github.com/gfm/#example-543
    test_identical_markdown_events!(r##"[foo *bar][ref]

[ref]: /uri"##);
}

#[test]
fn gfm_markdown_links_544() {
    // https://github.github.com/gfm/#example-544
    test_identical_markdown_events!(r##"[foo <bar attr="][ref]">

[ref]: /uri"##);
}

#[test]
fn gfm_markdown_links_545() {
    // https://github.github.com/gfm/#example-545
    test_identical_markdown_events!(r##"[foo`][ref]`

[ref]: /uri"##);
}

#[test]
fn gfm_markdown_links_546() {
    // https://github.github.com/gfm/#example-546
    test_identical_markdown_events!(r##"[foo<http://example.com/?search=][ref]>

[ref]: /uri"##);
}

#[test]
fn gfm_markdown_links_547() {
    // https://github.github.com/gfm/#example-547
    test_identical_markdown_events!(r##"[foo][BaR]

[bar]: /url "title""##);
}

#[test]
fn gfm_markdown_links_548() {
    // https://github.github.com/gfm/#example-548
    test_identical_markdown_events!(r##"[Толпой][Толпой] is a Russian word.

[ТОЛПОЙ]: /url"##);
}

#[test]
fn gfm_markdown_links_549() {
    // https://github.github.com/gfm/#example-549
    test_identical_markdown_events!(r##"[Foo
  bar]: /url

[Baz][Foo bar]"##,r##"[Foo bar]: /url

[Baz][Foo bar]"##);
}

#[test]
fn gfm_markdown_links_550() {
    // https://github.github.com/gfm/#example-550
    test_identical_markdown_events!(r##"[foo] [bar]

[bar]: /url "title""##);
}

#[test]
fn gfm_markdown_links_551() {
    // https://github.github.com/gfm/#example-551
    test_identical_markdown_events!(r##"[foo]
[bar]

[bar]: /url "title""##);
}

#[test]
fn gfm_markdown_links_552() {
    // https://github.github.com/gfm/#example-552
    test_identical_markdown_events!(r##"[foo]: /url1

[foo]: /url2

[bar][foo]"##,r##"[foo]: /url1



[bar][foo]"##);
}

#[test]
fn gfm_markdown_links_553() {
    // https://github.github.com/gfm/#example-553
    test_identical_markdown_events!(r##"[bar][foo\!]

[foo!]: /url"##);
}

#[test]
fn gfm_markdown_links_554() {
    // https://github.github.com/gfm/#example-554
    test_identical_markdown_events!(r##"[foo][ref[]

[ref[]: /uri"##);
}

#[test]
fn gfm_markdown_links_555() {
    // https://github.github.com/gfm/#example-555
    test_identical_markdown_events!(r##"[foo][ref[bar]]

[ref[bar]]: /uri"##);
}

#[test]
fn gfm_markdown_links_556() {
    // https://github.github.com/gfm/#example-556
    test_identical_markdown_events!(r##"[[[foo]]]

[[[foo]]]: /url"##);
}

#[test]
fn gfm_markdown_links_557() {
    // https://github.github.com/gfm/#example-557
    test_identical_markdown_events!(r##"[foo][ref\[]

[ref\[]: /uri"##);
}

#[test]
fn gfm_markdown_links_558() {
    // https://github.github.com/gfm/#example-558
    test_identical_markdown_events!(r##"[bar\\]: /uri

[bar\\]"##);
}

#[test]
fn gfm_markdown_links_559() {
    // https://github.github.com/gfm/#example-559
    test_identical_markdown_events!(r##"[]

[]: /uri"##);
}

#[test]
fn gfm_markdown_links_560() {
    // https://github.github.com/gfm/#example-560
    test_identical_markdown_events!(r##"[
 ]

[
 ]: /uri"##,r##"[
]

[
]: /uri"##);
}

#[test]
fn gfm_markdown_links_561() {
    // https://github.github.com/gfm/#example-561
    test_identical_markdown_events!(r##"[foo][]

[foo]: /url "title""##);
}

#[test]
fn gfm_markdown_links_562() {
    // https://github.github.com/gfm/#example-562
    test_identical_markdown_events!(r##"[*foo* bar][]

[*foo* bar]: /url "title""##);
}

#[test]
fn gfm_markdown_links_563() {
    // https://github.github.com/gfm/#example-563
    test_identical_markdown_events!(r##"[Foo][]

[foo]: /url "title""##);
}

#[test]
fn gfm_markdown_links_564() {
    // https://github.github.com/gfm/#example-564
    test_identical_markdown_events!("[foo] \n[]\n\n[foo]: /url \"title\"");
}

#[test]
fn gfm_markdown_links_565() {
    // https://github.github.com/gfm/#example-565
    test_identical_markdown_events!(r##"[foo]

[foo]: /url "title""##);
}

#[test]
fn gfm_markdown_links_566() {
    // https://github.github.com/gfm/#example-566
    test_identical_markdown_events!(r##"[*foo* bar]

[*foo* bar]: /url "title""##);
}

#[test]
fn gfm_markdown_links_567() {
    // https://github.github.com/gfm/#example-567
    test_identical_markdown_events!(r##"[[*foo* bar]]

[*foo* bar]: /url "title""##);
}

#[test]
fn gfm_markdown_links_568() {
    // https://github.github.com/gfm/#example-568
    test_identical_markdown_events!(r##"[[bar [foo]

[foo]: /url"##);
}

#[test]
fn gfm_markdown_links_569() {
    // https://github.github.com/gfm/#example-569
    test_identical_markdown_events!(r##"[Foo]

[foo]: /url "title""##);
}

#[test]
fn gfm_markdown_links_570() {
    // https://github.github.com/gfm/#example-570
    test_identical_markdown_events!(r##"[foo] bar

[foo]: /url"##);
}

#[test]
fn gfm_markdown_links_571() {
    // https://github.github.com/gfm/#example-571
    test_identical_markdown_events!(r##"\[foo]

[foo]: /url "title""##);
}

#[test]
fn gfm_markdown_links_572() {
    // https://github.github.com/gfm/#example-572
    test_identical_markdown_events!(r##"[foo*]: /url

*[foo*]"##);
}

#[test]
fn gfm_markdown_links_573() {
    // https://github.github.com/gfm/#example-573
    test_identical_markdown_events!(r##"[foo][bar]

[foo]: /url1
[bar]: /url2"##);
}

#[test]
fn gfm_markdown_links_574() {
    // https://github.github.com/gfm/#example-574
    test_identical_markdown_events!(r##"[foo][]

[foo]: /url1"##);
}

#[test]
fn gfm_markdown_links_575() {
    // https://github.github.com/gfm/#example-575
    test_identical_markdown_events!(r##"[foo]()

[foo]: /url1"##);
}

#[test]
fn gfm_markdown_links_576() {
    // https://github.github.com/gfm/#example-576
    test_identical_markdown_events!(r##"[foo](not a link)

[foo]: /url1"##);
}

#[test]
fn gfm_markdown_links_577() {
    // https://github.github.com/gfm/#example-577
    test_identical_markdown_events!(r##"[foo][bar][baz]

[baz]: /url"##);
}

#[test]
fn gfm_markdown_links_578() {
    // https://github.github.com/gfm/#example-578
    test_identical_markdown_events!(r##"[foo][bar][baz]

[baz]: /url1
[bar]: /url2"##);
}

#[test]
fn gfm_markdown_links_579() {
    // https://github.github.com/gfm/#example-579
    test_identical_markdown_events!(r##"[foo][bar][baz]

[baz]: /url1
[foo]: /url2"##);
}

#[test]
fn gfm_markdown_images_580() {
    // https://github.github.com/gfm/#example-580
    test_identical_markdown_events!(r##"![foo](/url "title")"##);
}

#[test]
fn gfm_markdown_images_581() {
    // https://github.github.com/gfm/#example-581
    test_identical_markdown_events!(r##"![foo *bar*]

[foo *bar*]: train.jpg "train & tracks""##);
}

#[test]
fn gfm_markdown_images_582() {
    // https://github.github.com/gfm/#example-582
    test_identical_markdown_events!(r##"![foo ![bar](/url)](/url2)"##);
}

#[test]
fn gfm_markdown_images_583() {
    // https://github.github.com/gfm/#example-583
    test_identical_markdown_events!(r##"![foo [bar](/url)](/url2)"##);
}

#[test]
fn gfm_markdown_images_584() {
    // https://github.github.com/gfm/#example-584
    test_identical_markdown_events!(r##"![foo *bar*][]

[foo *bar*]: train.jpg "train & tracks""##);
}

#[test]
fn gfm_markdown_images_585() {
    // https://github.github.com/gfm/#example-585
    test_identical_markdown_events!(r##"![foo *bar*][foobar]

[FOOBAR]: train.jpg "train & tracks""##);
}

#[test]
fn gfm_markdown_images_586() {
    // https://github.github.com/gfm/#example-586
    test_identical_markdown_events!(r##"![foo](train.jpg)"##);
}

#[test]
fn gfm_markdown_images_587() {
    // https://github.github.com/gfm/#example-587
    test_identical_markdown_events!(r##"My ![foo bar](/path/to/train.jpg  "title"   )"##,r##"My ![foo bar](/path/to/train.jpg "title")"##);
}

#[test]
fn gfm_markdown_images_588() {
    // https://github.github.com/gfm/#example-588
    test_identical_markdown_events!(r##"![foo](<url>)"##,r##"![foo](url)"##);
}

#[test]
fn gfm_markdown_images_589() {
    // https://github.github.com/gfm/#example-589
    test_identical_markdown_events!(r##"![](/url)"##);
}

#[test]
fn gfm_markdown_images_590() {
    // https://github.github.com/gfm/#example-590
    test_identical_markdown_events!(r##"![foo][bar]

[bar]: /url"##);
}

#[test]
fn gfm_markdown_images_591() {
    // https://github.github.com/gfm/#example-591
    test_identical_markdown_events!(r##"![foo][bar]

[BAR]: /url"##);
}

#[test]
fn gfm_markdown_images_592() {
    // https://github.github.com/gfm/#example-592
    test_identical_markdown_events!(r##"![foo][]

[foo]: /url "title""##);
}

#[test]
fn gfm_markdown_images_593() {
    // https://github.github.com/gfm/#example-593
    test_identical_markdown_events!(r##"![*foo* bar][]

[*foo* bar]: /url "title""##);
}

#[test]
fn gfm_markdown_images_594() {
    // https://github.github.com/gfm/#example-594
    test_identical_markdown_events!(r##"![Foo][]

[foo]: /url "title""##);
}

#[test]
fn gfm_markdown_images_595() {
    // https://github.github.com/gfm/#example-595
    test_identical_markdown_events!("![foo] \n[]\n\n[foo]: /url \"title\"");
}

#[test]
fn gfm_markdown_images_596() {
    // https://github.github.com/gfm/#example-596
    test_identical_markdown_events!(r##"![foo]

[foo]: /url "title""##);
}

#[test]
fn gfm_markdown_images_597() {
    // https://github.github.com/gfm/#example-597
    test_identical_markdown_events!(r##"![*foo* bar]

[*foo* bar]: /url "title""##);
}

#[test]
fn gfm_markdown_images_598() {
    // https://github.github.com/gfm/#example-598
    test_identical_markdown_events!(r##"![[foo]]

[[foo]]: /url "title""##);
}

#[test]
fn gfm_markdown_images_599() {
    // https://github.github.com/gfm/#example-599
    test_identical_markdown_events!(r##"![Foo]

[foo]: /url "title""##);
}

#[test]
fn gfm_markdown_images_600() {
    // https://github.github.com/gfm/#example-600
    test_identical_markdown_events!(r##"!\[foo]

[foo]: /url "title""##);
}

#[test]
fn gfm_markdown_images_601() {
    // https://github.github.com/gfm/#example-601
    test_identical_markdown_events!(r##"\![foo]

[foo]: /url "title""##);
}

#[test]
fn gfm_markdown_autolinks_602() {
    // https://github.github.com/gfm/#example-602
    test_identical_markdown_events!(r##"<http://foo.bar.baz>"##);
}

#[test]
fn gfm_markdown_autolinks_603() {
    // https://github.github.com/gfm/#example-603
    test_identical_markdown_events!(r##"<http://foo.bar.baz/test?q=hello&id=22&boolean>"##);
}

#[test]
fn gfm_markdown_autolinks_604() {
    // https://github.github.com/gfm/#example-604
    test_identical_markdown_events!(r##"<irc://foo.bar:2233/baz>"##);
}

#[test]
fn gfm_markdown_autolinks_605() {
    // https://github.github.com/gfm/#example-605
    test_identical_markdown_events!(r##"<MAILTO:FOO@BAR.BAZ>"##);
}

#[test]
fn gfm_markdown_autolinks_606() {
    // https://github.github.com/gfm/#example-606
    test_identical_markdown_events!(r##"<a+b+c:d>"##);
}

#[test]
fn gfm_markdown_autolinks_607() {
    // https://github.github.com/gfm/#example-607
    test_identical_markdown_events!(r##"<made-up-scheme://foo,bar>"##);
}

#[test]
fn gfm_markdown_autolinks_608() {
    // https://github.github.com/gfm/#example-608
    test_identical_markdown_events!(r##"<http://../>"##);
}

#[test]
fn gfm_markdown_autolinks_609() {
    // https://github.github.com/gfm/#example-609
    test_identical_markdown_events!(r##"<localhost:5001/foo>"##);
}

#[test]
fn gfm_markdown_autolinks_610() {
    // https://github.github.com/gfm/#example-610
    test_identical_markdown_events!(r##"<http://foo.bar/baz bim>"##);
}

#[test]
fn gfm_markdown_autolinks_611() {
    // https://github.github.com/gfm/#example-611
    test_identical_markdown_events!(r##"<http://example.com/\[\>"##);
}

#[test]
fn gfm_markdown_autolinks_612() {
    // https://github.github.com/gfm/#example-612
    test_identical_markdown_events!(r##"<foo@bar.example.com>"##);
}

#[test]
fn gfm_markdown_autolinks_613() {
    // https://github.github.com/gfm/#example-613
    test_identical_markdown_events!(r##"<foo+special@Bar.baz-bar0.com>"##);
}

#[test]
fn gfm_markdown_autolinks_614() {
    // https://github.github.com/gfm/#example-614
    test_identical_markdown_events!(r##"<foo\+@bar.example.com>"##);
}

#[test]
fn gfm_markdown_autolinks_615() {
    // https://github.github.com/gfm/#example-615
    test_identical_markdown_events!(r##"<>"##);
}

#[test]
fn gfm_markdown_autolinks_616() {
    // https://github.github.com/gfm/#example-616
    test_identical_markdown_events!(r##"< http://foo.bar >"##);
}

#[test]
fn gfm_markdown_autolinks_617() {
    // https://github.github.com/gfm/#example-617
    test_identical_markdown_events!(r##"<m:abc>"##);
}

#[test]
fn gfm_markdown_autolinks_618() {
    // https://github.github.com/gfm/#example-618
    test_identical_markdown_events!(r##"<foo.bar.baz>"##);
}

#[test]
fn gfm_markdown_autolinks_619() {
    // https://github.github.com/gfm/#example-619
    test_identical_markdown_events!(r##"http://example.com"##);
}

#[test]
fn gfm_markdown_autolinks_620() {
    // https://github.github.com/gfm/#example-620
    test_identical_markdown_events!(r##"foo@bar.example.com"##);
}

#[test]
fn gfm_markdown_autolinks_extension_621() {
    // https://github.github.com/gfm/#example-621
    test_identical_markdown_events!(r##"www.commonmark.org"##);
}

#[test]
fn gfm_markdown_autolinks_extension_622() {
    // https://github.github.com/gfm/#example-622
    test_identical_markdown_events!(r##"Visit www.commonmark.org/help for more information."##);
}

#[test]
fn gfm_markdown_autolinks_extension_623() {
    // https://github.github.com/gfm/#example-623
    test_identical_markdown_events!(r##"Visit www.commonmark.org.

Visit www.commonmark.org/a.b."##);
}

#[test]
fn gfm_markdown_autolinks_extension_624() {
    // https://github.github.com/gfm/#example-624
    test_identical_markdown_events!(r##"www.google.com/search?q=Markup+(business)

www.google.com/search?q=Markup+(business)))

(www.google.com/search?q=Markup+(business))

(www.google.com/search?q=Markup+(business)"##);
}

#[test]
fn gfm_markdown_autolinks_extension_625() {
    // https://github.github.com/gfm/#example-625
    test_identical_markdown_events!(r##"www.google.com/search?q=(business))+ok"##);
}

#[test]
fn gfm_markdown_autolinks_extension_626() {
    // https://github.github.com/gfm/#example-626
    test_identical_markdown_events!(r##"www.google.com/search?q=commonmark&hl=en

www.google.com/search?q=commonmark&hl;"##);
}

#[test]
fn gfm_markdown_autolinks_extension_627() {
    // https://github.github.com/gfm/#example-627
    test_identical_markdown_events!(r##"www.commonmark.org/he<lp"##);
}

#[test]
fn gfm_markdown_autolinks_extension_628() {
    // https://github.github.com/gfm/#example-628
    test_identical_markdown_events!(r##"http://commonmark.org

(Visit https://encrypted.google.com/search?q=Markup+(business))

Anonymous FTP is available at ftp://foo.bar.baz."##);
}

#[test]
fn gfm_markdown_autolinks_extension_629() {
    // https://github.github.com/gfm/#example-629
    test_identical_markdown_events!(r##"foo@bar.baz"##);
}

#[test]
fn gfm_markdown_autolinks_extension_630() {
    // https://github.github.com/gfm/#example-630
    test_identical_markdown_events!(r##"hello@mail+xyz.example isn't valid, but hello+xyz@mail.example is."##);
}

#[test]
fn gfm_markdown_autolinks_extension_631() {
    // https://github.github.com/gfm/#example-631
    test_identical_markdown_events!(r##"a.b-c_d@a.b

a.b-c_d@a.b.

a.b-c_d@a.b-

a.b-c_d@a.b_"##);
}

#[test]
fn gfm_markdown_raw_html_632() {
    // https://github.github.com/gfm/#example-632
    test_identical_markdown_events!(r##"<a><bab><c2c>"##);
}

#[test]
fn gfm_markdown_raw_html_633() {
    // https://github.github.com/gfm/#example-633
    test_identical_markdown_events!(r##"<a/><b2/>"##);
}

#[test]
fn gfm_markdown_raw_html_634() {
    // https://github.github.com/gfm/#example-634
    test_identical_markdown_events!(r##"<a  /><b2
data="foo" >"##);
}

#[test]
fn gfm_markdown_raw_html_635() {
    // https://github.github.com/gfm/#example-635
    test_identical_markdown_events!(r##"<a foo="bar" bam = 'baz <em>"</em>'
_boolean zoop:33=zoop:33 />"##);
}

#[test]
fn gfm_markdown_raw_html_636() {
    // https://github.github.com/gfm/#example-636
    test_identical_markdown_events!(r##"Foo <responsive-image src="foo.jpg" />"##);
}

#[test]
fn gfm_markdown_raw_html_637() {
    // https://github.github.com/gfm/#example-637
    test_identical_markdown_events!(r##"<33> <__>"##);
}

#[test]
fn gfm_markdown_raw_html_638() {
    // https://github.github.com/gfm/#example-638
    test_identical_markdown_events!(r##"<a h*#ref="hi">"##);
}

#[test]
fn gfm_markdown_raw_html_639() {
    // https://github.github.com/gfm/#example-639
    test_identical_markdown_events!(r##"<a href="hi'> <a href=hi'>"##);
}

#[test]
fn gfm_markdown_raw_html_640() {
    // https://github.github.com/gfm/#example-640
    test_identical_markdown_events!(r##"< a><
foo><bar/ >
<foo bar=baz
bim!bop />"##);
}

#[test]
fn gfm_markdown_raw_html_641() {
    // https://github.github.com/gfm/#example-641
    test_identical_markdown_events!(r##"<a href='bar'title=title>"##);
}

#[test]
fn gfm_markdown_raw_html_642() {
    // https://github.github.com/gfm/#example-642
    test_identical_markdown_events!(r##"</a></foo >"##);
}

#[test]
fn gfm_markdown_raw_html_643() {
    // https://github.github.com/gfm/#example-643
    test_identical_markdown_events!(r##"</a href="foo">"##);
}

#[test]
fn gfm_markdown_raw_html_644() {
    // https://github.github.com/gfm/#example-644
    test_identical_markdown_events!(r##"foo <!-- this is a --
comment - with hyphens -->"##);
}

#[test]
fn gfm_markdown_raw_html_645() {
    // https://github.github.com/gfm/#example-645
    test_identical_markdown_events!(r##"foo <!--> foo -->

foo <!---> foo -->"##);
}

#[test]
fn gfm_markdown_raw_html_646() {
    // https://github.github.com/gfm/#example-646
    test_identical_markdown_events!(r##"foo <?php echo $a; ?>"##);
}

#[test]
fn gfm_markdown_raw_html_647() {
    // https://github.github.com/gfm/#example-647
    test_identical_markdown_events!(r##"foo <!ELEMENT br EMPTY>"##);
}

#[test]
fn gfm_markdown_raw_html_648() {
    // https://github.github.com/gfm/#example-648
    test_identical_markdown_events!(r##"foo <![CDATA[>&<]]>"##);
}

#[test]
fn gfm_markdown_raw_html_649() {
    // https://github.github.com/gfm/#example-649
    test_identical_markdown_events!(r##"foo <a href="&ouml;">"##);
}

#[test]
fn gfm_markdown_raw_html_650() {
    // https://github.github.com/gfm/#example-650
    test_identical_markdown_events!(r##"foo <a href="\*">"##);
}

#[test]
fn gfm_markdown_raw_html_651() {
    // https://github.github.com/gfm/#example-651
    test_identical_markdown_events!(r##"<a href="\"">"##);
}

#[test]
fn gfm_markdown_disallowed_raw_html_extension_652() {
    // https://github.github.com/gfm/#example-652
    test_identical_markdown_events!(r##"<strong> <title> <style> <em>

<blockquote>
  <xmp> is disallowed.  <XMP> is also disallowed.
</blockquote>"##);
}

#[test]
fn gfm_markdown_hard_line_breaks_653() {
    // https://github.github.com/gfm/#example-653
    test_identical_markdown_events!("foo  \nbaz");
}

#[test]
fn gfm_markdown_hard_line_breaks_654() {
    // https://github.github.com/gfm/#example-654
    test_identical_markdown_events!(r##"foo\
baz"##);
}

#[test]
fn gfm_markdown_hard_line_breaks_655() {
    // https://github.github.com/gfm/#example-655
    test_identical_markdown_events!("foo       \nbaz");
}

#[test]
fn gfm_markdown_hard_line_breaks_656() {
    // https://github.github.com/gfm/#example-656
    test_identical_markdown_events!("foo  \n     bar","foo  \nbar");
}

#[test]
fn gfm_markdown_hard_line_breaks_657() {
    // https://github.github.com/gfm/#example-657
    test_identical_markdown_events!(r##"foo\
     bar"##,r##"foo\
bar"##);
}

#[test]
fn gfm_markdown_hard_line_breaks_658() {
    // https://github.github.com/gfm/#example-658
    test_identical_markdown_events!("*foo  \nbar*");
}

#[test]
fn gfm_markdown_hard_line_breaks_659() {
    // https://github.github.com/gfm/#example-659
    test_identical_markdown_events!(r##"*foo\
bar*"##);
}

#[test]
fn gfm_markdown_hard_line_breaks_660() {
    // https://github.github.com/gfm/#example-660
    test_identical_markdown_events!("`code  \nspan`");
}

#[test]
fn gfm_markdown_hard_line_breaks_661() {
    // https://github.github.com/gfm/#example-661
    test_identical_markdown_events!(r##"`code\
span`"##);
}

#[test]
fn gfm_markdown_hard_line_breaks_662() {
    // https://github.github.com/gfm/#example-662
    test_identical_markdown_events!("<a href=\"foo  \nbar\">");
}

#[test]
fn gfm_markdown_hard_line_breaks_663() {
    // https://github.github.com/gfm/#example-663
    test_identical_markdown_events!(r##"<a href="foo\
bar">"##);
}

#[test]
fn gfm_markdown_hard_line_breaks_664() {
    // https://github.github.com/gfm/#example-664
    test_identical_markdown_events!(r##"foo\"##);
}

#[test]
fn gfm_markdown_hard_line_breaks_665() {
    // https://github.github.com/gfm/#example-665
    test_identical_markdown_events!("foo  ");
}

#[test]
fn gfm_markdown_hard_line_breaks_666() {
    // https://github.github.com/gfm/#example-666
    test_identical_markdown_events!(r##"### foo\"##);
}

#[test]
fn gfm_markdown_hard_line_breaks_667() {
    // https://github.github.com/gfm/#example-667
    test_identical_markdown_events!("### foo  ",r##"### foo"##);
}

#[test]
fn gfm_markdown_soft_line_breaks_668() {
    // https://github.github.com/gfm/#example-668
    test_identical_markdown_events!(r##"foo
baz"##);
}

#[test]
fn gfm_markdown_soft_line_breaks_669() {
    // https://github.github.com/gfm/#example-669
    test_identical_markdown_events!("foo \n baz","foo \nbaz");
}

#[test]
fn gfm_markdown_textual_content_670() {
    // https://github.github.com/gfm/#example-670
    test_identical_markdown_events!(r##"hello $.;'there"##);
}

#[test]
fn gfm_markdown_textual_content_671() {
    // https://github.github.com/gfm/#example-671
    test_identical_markdown_events!(r##"Foo χρῆν"##);
}

#[test]
fn gfm_markdown_textual_content_672() {
    // https://github.github.com/gfm/#example-672
    test_identical_markdown_events!(r##"Multiple     spaces"##);
}
