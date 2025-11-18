
## 1️⃣ Create parent folder with children

```sh
mkdir -p parent/child1 parent/child2
cd parent
pwd
```

✔ Output MUST end with `/parent`

---

## 2️⃣ Return to HOME using `cd`

```sh
cd
pwd
```

✔ Output MUST show home path

---

## 3️⃣ List current directory contents

```sh
ls
```

---

## 4️⃣ Test all ls flags

```sh
ls -l -a -F
```

✔ Must show:

- Hidden files
- `/` after directories
- Long listing

---

## 5️⃣ Create two folders

```sh
mkdir new_folder1
mkdir new_folder2
ls
```

---

## 6️⃣ Create a text file with content

```sh
echo RANDOM-CONTENT > new_folder1/new_doc.txt
cat new_folder1/new_doc.txt
```

✔ Must print:

```
RANDOM-CONTENT
```

---

## 7️⃣ Copy file to folder2

### Option A (from shell root)

```sh
cp new_folder1/new_doc.txt new_folder2
```

### Option B (inside new_folder1)

```sh
cd new_folder1
cp new_doc.txt ../new_folder2
cd ..
```

Verify:

```sh
ls new_folder2
cat new_folder2/new_doc.txt
```

---

## 8️⃣ Move folder2 inside folder1

```sh
mv new_folder2 new_folder1
ls new_folder1
```

---

## 9️⃣ Remove everything

```sh
rm -r new_folder1
ls
```

---

## 🔟 Test standard builtins

```sh
echo hello world
pwd
history
help
```

---

## 1️⃣1️⃣ Test pipeline and redirection

```sh
echo hi | cat - > out.txt
cat out.txt
```

✔ Must print:

```
hi
```

---

## 1️⃣2️⃣ Test HOME expansion

```sh
echo ~
echo ~/folder
```

---


