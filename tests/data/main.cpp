#include<stdio.h>\n#include<string.h>\nint main()\n{\n\tchar a[110],b[110],ch,q,p;\n\tint i,count,j;\n\t\tfor(i=0;;i++)\n\t\t{\n\t\t\tscanf(\"%c\",&a[i]);\n\t\t\tif(a[i]=='.')\n\t\t\t\tbreak;\n\t\t}\n\t\ta[i+1]='\\0';\n\t\tgetchar();\n\t\tscanf(\"%c\",&ch);\n\t\tgetchar();\n\t\tif(ch=='D')\n\t\t\tscanf(\"%c\",&q);\n\t\telse\n\t\t\tscanf(\"%c %c\",&q,&p);\n\t\tgetchar();\n\t\tif(ch=='D')\n\t\t{\n\t\t\tcount=0,j=0;\n\t\t\tfor(i=0;i<strlen(a);i++)\n\t\t\t{\n\t\t\t\tif(a[i]==q&&count==0)\n\t\t\t\t{\n\t\t\t\t\tcount++;\n\t\t\t\t}\n\t\t\t\telse\n\t\t\t\t{\n\t\t\t\t\tb[j++]=a[i];\n\t\t\t\t}\n\t\t\t}\n\t\t\tb[j]='\\0';\n\t\t\tif(count==0)\n\t\t\t\tprintf(\"Not exist\\n\");\n\t\t\telse\n\t\t\t    printf(\"%s\\n\",b);\n\t\t}\n\t\tif(ch=='I')\n\t\t{\n\t\t\tcount=0;j=strlen(a);\n\t\t\tfor(i=strlen(a)-1;i>=0;i--)\n\t\t\t{\n\t\t\t\tif(a[i]==q&&count==0)\n\t\t\t\t{\n\t\t\t\t\tcount++;\n\t\t\t\t\tb[j--]=a[i];\n\t\t\t\t\tb[j--]=p;\n\t\t\t\t}\n\t\t\t\telse\n\t\t\t\t{\n\t\t\t\t\tb[j--]=a[i];\n\t\t\t\t}\n\t\t\t}\n\t\t\tb[strlen(a)+1]='\\0';\n\t\t\tif(count==0)\n\t\t\t\tprintf(\"Not exist\\n\");\n\t\t\telse\n\t\t\t    printf(\"%s\\n\",b);\n\t\t}\n\t\tif(ch=='R')\n\t\t{\n\t\t\tcount=0,j=0;\n\t\t\tfor(i=0;i<strlen(a);i++)\n\t\t\t{\n\t\t\t\tif(a[i]==q)\n\t\t\t\t{\n\t\t\t\t\tb[j++]=p;\n\t\t\t\t\tcount++;\n\t\t\t\t}\n\t\t\t\telse\n\t\t\t\t\tb[j++]=a[i];\n\t\t\t}\n\t\t\tb[j]='\\0';\n\t\t\tif(count==0)\n\t\t\t\tprintf(\"Not exist\\n\");\n\t\t\telse\n\t\t\t\tprintf(\"%s\\n\",b);\n\t\t}\n\treturn 0;\n}