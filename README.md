# Sort_CHS_With_PinYin-rs
Sort file contents with pinyin and auto drop duplicates.

将给出的文件内容使用拼音进行排序，在自动去重后输出到同目录中，加后缀 `_out`

## 示例：

`./chs_sentences_py_sort.exe "d:\desktop\测试 文件.txt"`

将会读取 `测试 文件.txt`

在去除重复的行以后将各行按拼音顺序排序，并输出到
`"d:\desktop\测试 文件_out.txt"`中
