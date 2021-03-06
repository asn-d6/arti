// List of directories on the Tor network where we think we can
// download directory info.  We only use this list when we haven't yet
// downloaded our first directory.
//
// This list is derived from Tor's list in Nov 2020, but any nodes that
// were down at the time were removed.

vec![
    fallback(
        "0338F9F55111FE8E3570E7DE117EF3AF999CC1D7",
        "GsyBNDtZ8A3X++jOPpNWDJaDXFetH0qbXaqzxp3vWbI",
        vec!["185.225.17.3:443", "[2a0a:c800:1:5::3]:443"],
    ),
    fallback(
        "03C3069E814E296EB18776EB61B1ECB754ED89FE",
        "5nP3zlhsox/Q3U9etQ+zrWINQPNzc8OOwV/kbnK4NZU",
        vec!["81.7.10.193:993"],
    ),
    fallback(
        "0C475BA4D3AA3C289B716F95954CAD616E50C4E5",
        "dio9Dfc+qWuvUnJJERmEflb8qycXnrXVkjS4ecXhZrs",
        vec!["81.7.18.7:9001", "[2a02:180:6:1::2eee]:9001"],
    ),
    fallback(
        "0F6E5CA4BF5565D9AA9FDDCA165AFC6A5305763D",
        "1FdOnRvYuHpggwoSjbhiAk9oo2JUCGYTfAyMHRATCzM",
        vec![
            "193.234.15.60:443",
            "[2a00:1c20:4089:1234:67bc:79f3:61c0:6e49]:443",
        ],
    ),
    fallback(
        "113143469021882C3A4B82F084F8125B08EE471E",
        "TMzMEPui071YlaOG8EzlT9a5YrlI//59DL+5VRn8d5k",
        vec!["93.177.67.71:8080", "[2a03:4000:38:559::2]:8080"],
    ),
    fallback(
        "11DF0017A43AF1F08825CD5D973297F81AB00FF3",
        "fgDwY4AcdAv5DM05wam5hoD0jslTeU3Q8ETAx4GPGvk",
        vec![
            "37.120.174.249:443",
            "[2a03:4000:6:724c:df98:15f9:b34d:443]:443",
        ],
    ),
    fallback(
        "12AD30E5D25AA67F519780E2111E611A455FDC89",
        "fejD+zy4mgihice7QP/LvGdrUdJabMwGa+lmNF9Kp+Q",
        vec!["193.11.114.43:9001", "[2001:6b0:30:1000::99]:9050"],
    ),
    fallback(
        "12FD624EE73CEF37137C90D38B2406A66F68FAA2",
        "qoVxOXoWz+T+ngAf32fIQDCwTKCSGeFj0rZNE1YyqP0",
        vec!["37.157.195.87:443"],
    ),
    fallback(
        "158581827034DEF1BAB1FC248D180165452E53D3",
        "gthdSWvrzfj1SJ5LUDc6Xuxv6wLGvlSa7YnZrZPSLBM",
        vec![
            "193.234.15.61:443",
            "[2a00:1c20:4089:1234:2712:a3d0:666b:88a6]:443",
        ],
    ),
    fallback(
        "15BE17C99FACE24470D40AF782D6A9C692AB36D6",
        "DPvwZRCFlEXzfRT1SBe+hIyiP0UeW2J2wM/HHQeSFn8",
        vec!["51.15.78.0:9001", "[2001:bc8:1824:c4b::1]:9001"],
    ),
    fallback(
        "185F2A57B0C4620582602761097D17DB81654F70",
        "bAQa7y4SFhpgb4B0zom+mJoHqfnyL5tbt2RbSV18Idc",
        vec!["204.11.50.131:9001"],
    ),
    fallback(
        "1CD17CB202063C51C7DAD3BACEF87ECE81C2350F",
        "p2d7Z7R3PCK3Y/DDQTNBaXdksASEdTaD+V7I2BIWWKI",
        vec!["50.7.74.171:9001", "[2001:49f0:d002:2::51]:443"],
    ),
    fallback(
        "1F6ABD086F40B890A33C93CC4606EE68B31C9556",
        "DcZZHUOmJ+FNRj7xgecpAML/sjGBu4JYZr3CceHDL88",
        vec!["199.184.246.250:443", "[2620:124:1009:1::171]:443"],
    ),
    fallback(
        "20462CBA5DA4C2D963567D17D0B7249718114A68",
        "Z/yFHI658azdnKU3Dz0Susfnc4Y0CKmpP7jzqoyNxp4",
        vec!["212.47.229.2:9001", "[2001:bc8:47ac:23a::1]:9001"],
    ),
    fallback(
        "204DFD2A2C6A0DC1FA0EACB495218E0B661704FD",
        "O2ddyzT3VejqK7/rH2WM6iUR2HeGUf3NYGdNYj3LFIM",
        vec!["77.247.181.164:443"],
    ),
    fallback(
        "24D0491A2ADAAB52C17625FBC926D84477AEA322",
        "r7vuNUryeNm+Qw9886C9uE4o5rBYCXPxhA3I+gpHGzU",
        vec![
            "193.234.15.57:443",
            "[2a00:1c20:4089:1234:7825:2c5d:1ecd:c66f]:443",
        ],
    ),
    fallback(
        "28F4F392F8F19E3FBDE09616D9DB8143A1E2DDD3",
        "4M/pCad2pYGL7OJUeRWJYE8S1f32QUVY0iLoNKb8T5Q",
        vec!["185.220.101.137:10137", "[2a0b:f4c2:1::137]:10137"],
    ),
    fallback(
        "2BA2C8E96B2590E1072AECE2BDB5C48921BF8510",
        "0c93jN2vfAgY9dryGcGu/OahU6MVroE3FRU0aKpoxc0",
        vec!["138.201.250.33:9011"],
    ),
    fallback(
        "2BB85DC5BD3C6F0D81A4F2B5882176C6BF7ECF5A",
        "Nw2I+2Lsv3CX8jEY9GJkf1jGkbq6a2zDaMn0G10rjkk",
        vec![
            "5.181.50.99:443",
            "[2a03:4000:3f:16c:3851:6bff:fe07:bd2]:443",
        ],
    ),
    fallback(
        "2F0F32AB1E5B943CA7D062C03F18960C86E70D94",
        "CphtNy4/DfqEe5XX9NWzR15VdhfYvtQL42+cCxr2o/I",
        vec!["97.74.237.196:9001"],
    ),
    fallback(
        "311A4533F7A2415F42346A6C8FA77E6FD279594C",
        "gjYpaXjgfL6R5H0L5/kPC7KRQReaQ/HXTqRbSI1H+PQ",
        vec!["94.230.208.147:8443", "[2a02:418:6017::147]:8443"],
    ),
    fallback(
        "32EE911D968BE3E016ECA572BB1ED0A9EE43FC2F",
        "VsG0z6PR8T7s3HsOSBvBgD6WL4NBa219n04u+Qc/9RQ",
        vec!["109.105.109.162:60784", "[2001:948:7:2::163]:5001"],
    ),
    fallback(
        "330CD3DB6AD266DC70CDB512B036957D03D9BC59",
        "zV1cBIQeQNP+D7E/o7QFwZkR656FnThzbHMXNIQlflk",
        vec!["185.100.84.212:443", "[2a06:1700:0:7::1]:443"],
    ),
    fallback(
        "375DCBB2DBD94E5263BC0C015F0C9E756669617E",
        "gERhugbL6dbAvWTQCEIozBw9QSEH5fTuqWbb8Jtjyc0",
        vec!["64.79.152.132:443"],
    ),
    fallback(
        "39F096961ED2576975C866D450373A9913AFDC92",
        "8fN2IHdgryRlXZbRLcvvBhMaYDRo8zCAIE0d/C+wOpU",
        vec!["198.50.191.95:443"],
    ),
    fallback(
        "3AFDAAD91A15B4C6A7686A53AA8627CA871FF491",
        "nbArRRyMh7DBRLr7XZuYP4hmZH4XVJ1gXc0nMZTNQZQ",
        vec!["50.7.74.174:9001", "[2001:49f0:d002:2::57]:443"],
    ),
    fallback(
        "3C79699D4FBC37DE1A212D5033B56DAE079AC0EF",
        "ZiTuTLaLEQjxq9bOD9c+dXAQhay6iAmUhs3DOmG+C40",
        vec!["212.83.154.33:443", "[2001:bc8:31d3:1dd::1]:443"],
    ),
    fallback(
        "3CB4193EF4E239FCEDC4DC43468E0B0D6B67ACC3",
        "3c5koajx8IV5RkBuLlll7KonUyZfmkHjeTYk0sijGdw",
        vec!["51.38.65.160:9001", "[2001:41d0:801:2000::f6e]:9001"],
    ),
    fallback(
        "3CCF9573F59137E52787D9C322AC19D2BD090B70",
        "onVQbM8ogjPrUyC9b3uW88D/u3kvkrJS3hC0xkSWgFE",
        vec!["95.216.211.81:443", "[2a01:4f9:c010:4dfa::1]:443"],
    ),
    fallback(
        "3E53D3979DB07EFD736661C934A1DED14127B684",
        "extACn0DLTFq98Ni2q+XtvqFgkv0Sti0T1fK9kTaBqk",
        vec![
            "217.79.179.177:9001",
            "[2001:4ba0:fff9:131:6c4f::90d3]:9001",
        ],
    ),
    fallback(
        "4061C553CA88021B8302F0814365070AAE617270",
        "xBGINMKnI6dvQLVPgb6bhzuzaIGvLw2Cpy2ZyZYmON4",
        vec!["185.100.85.101:9001"],
    ),
    fallback(
        "4623A9EC53BFD83155929E56D6F7B55B5E718C24",
        "TeDcUhWw+XxhYPN/bDJqvxYL7yg6p8cPmRLa5UOB9Ho",
        vec!["163.172.157.213:443"],
    ),
    fallback(
        "484A10BA2B8D48A5F0216674C8DD50EF27BC32F3",
        "sVTDglBln5kKGfpYI9bhRh8dNneCQsNjtCxoKUwqg7Y",
        vec!["193.70.43.76:9001"],
    ),
    fallback(
        "4BFC9C631A93FF4BA3AA84BC6931B4310C38A263",
        "xbDOU2WblscuIxu56QULiZ3zXXyL1vqf/lvmHECtgg8",
        vec!["109.70.100.4:443", "[2a03:e600:100::4]:443"],
    ),
    fallback(
        "4F0DB7E687FC7C0AE55C8F243DA8B0EB27FBF1F2",
        "7nRRGsJYe3/OEGWh1u92yJq836jaMIqsGUMCVuUfxGs",
        vec!["108.53.208.157:443"],
    ),
    fallback(
        "509EAB4C5D10C9A9A24B4EA0CE402C047A2D64E6",
        "O1MbXsxCb4A9rI/I6bnPI3Ds+XdncoMOSmTakJRAZrU",
        vec!["5.9.158.75:9001", "[2a01:4f8:190:514a::2]:9001"],
    ),
    fallback(
        "510176C07005D47B23E6796F02C93241A29AA0E9",
        "pkYjVpn6VIanyRogxcPemmlnL10ygM3tbTscSBiWfg8",
        vec!["69.30.215.42:443", "[2604:4300:a:2e:21b:21ff:fe11:392]:443"],
    ),
    fallback(
        "5262556D44A7F2434990FDE1AE7973C67DF49E58",
        "gAE5yi9lNflnXFFOVyzzN7wLXJ16oBrQ2btSqt5fI/g",
        vec!["176.223.141.106:443"],
    ),
    fallback(
        "52BFADA8BEAA01BA46C8F767F83C18E2FE50C1B9",
        "3Kvdb93+PpBhSP2sXLHQ4xdYAqyTN9YI7mY5pCi1LjQ",
        vec!["85.25.159.65:80"],
    ),
    fallback(
        "562434D987CF49D45649B76ADCA993BEA8F78471",
        "p8uVzfZwfOFIgd6XdUOJNL0PfRq/YmkpvMLTam383vg",
        vec![
            "193.234.15.59:443",
            "[2a00:1c20:4089:1234:bff6:e1bb:1ce3:8dc6]:443",
        ],
    ),
    fallback(
        "578E007E5E4535FBFEF7758D8587B07B4C8C5D06",
        "qm5XHTSZpVLD/fHmAUyUuAgzQszQFaFqLcIZXzExctM",
        vec!["89.234.157.254:443", "[2001:67c:2608::1]:443"],
    ),
    fallback(
        "5E56738E7F97AA81DEEF59AF28494293DFBFCCDF",
        "bLETd6FA4/k3KwSz9m01uC6MGdU9ZXtuOziW1vpgc+k",
        vec!["172.98.193.43:443"],
    ),
    fallback(
        "68F175CCABE727AA2D2309BCD8789499CEE36ED7",
        "sLGgkmWZCZAogV18gdFjBupGv3xma8cVuudnASz4v6Q",
        vec!["163.172.139.104:443"],
    ),
    fallback(
        "6A7551EEE18F78A9813096E82BF84F740D32B911",
        "798fdg7DGJ+m3YLNr9tuvcQCYe+OHdloWpttAhj+Vjc",
        vec!["95.217.16.212:443", "[2a01:4f9:c010:609a::1]:443"],
    ),
    fallback(
        "72B2B12A3F60408BDBC98C6DF53988D3A0B3F0EE",
        "PM9wXJmS/7VkrbNSSdq4nH7ig9VDRcTJYTu/qN1DYqo",
        vec!["85.235.250.88:443", "[2a01:3a0:1:1900:85:235:250:88]:443"],
    ),
    fallback(
        "742C45F2D9004AADE0077E528A4418A6A81BC2BA",
        "4Csh1Bs+xkBnbymEFzuND5JRXo05CbgaB5m+CH7WEGw",
        vec!["178.17.170.23:9001", "[2a00:1dc0:caff:7d::8254]:9001"],
    ),
    fallback(
        "7600680249A22080ECC6173FBBF64D6FCF330A61",
        "np89gXLD3ufDOjtjZBimLuRTiWhSdqX2Y1OTqJuUSLE",
        vec!["81.7.14.31:443"],
    ),
    fallback(
        "7614EF326635DA810638E2F5D449D10AE2BB7158",
        "TTkpUcQtaWnpzrRzV19jYQbSUyW5qd23+jnMHJoC9sU",
        vec!["62.171.144.155:443", "[2a02:c207:3004:8874::1]:443"],
    ),
    fallback(
        "77131D7E2EC1CA9B8D737502256DA9103599CE51",
        "V3itvRTciIQRaO+chK6IjeUbv/dCso9e/8Q41OIEuUc",
        vec!["77.247.181.166:443"],
    ),
    fallback(
        "775B0FAFDE71AADC23FFC8782B7BEB1D5A92733E",
        "hXPwzzXofHvVgMImcWAaJRIhudrOGPNvtCFlkYxMZNA",
        vec!["5.196.23.64:9001"],
    ),
    fallback(
        "79509683AB4C8DDAF90A120C69A4179C6CD5A387",
        "ou9JQi/To8KzH9VWO3DSqgjnWV2m63IDYH7zB1Hty5E",
        vec![
            "185.244.193.141:9001",
            "[2a03:4000:27:192:24:12:1984:4]:9001",
        ],
    ),
    fallback(
        "7BB70F8585DFC27E75D692970C0EEB0F22983A63",
        "iibuh5uywUapIP8+IytB9UJjopKVd73/BJf0yQvGazw",
        vec!["51.254.136.195:443"],
    ),
    fallback(
        "7BFB908A3AA5B491DA4CA72CCBEE0E1F2A939B55",
        "WkViAVHwT01UufFFaG2nHVOAwXqRrRDUOdiwbfB2B+w",
        vec!["77.247.181.162:443"],
    ),
    fallback(
        "80AAF8D5956A43C197104CEF2550CD42D165C6FB",
        "TkKHt6XxoQh6urm/xudkFG51kp4bs9RnQRyjbzvUAQc",
        vec!["193.11.114.45:9002"],
    ),
    fallback(
        "8101421BEFCCF4C271D5483C5AABCAAD245BBB9D",
        "dsg5Mu1OJzYpttfMDYSWF4dxLEdFivvbxYXmq6AeBpY",
        vec!["51.254.96.208:9001", "[2001:41d0:401:3100::30dc]:9001"],
    ),
    fallback(
        "8111FEB45EF2950EB8F84BFD8FF070AB07AEE9DD",
        "96u8ADx3igfVLI6mHJ4HCNmp7UgZDFPTxi3DxBogv9k",
        vec![
            "152.89.106.147:9001",
            "[2a03:4000:39:605:c4f2:c9ff:fe64:c215]:9001",
        ],
    ),
    fallback(
        "81B75D534F91BFB7C57AB67DA10BCEF622582AE8",
        "r4SmC/yRVvtx+IAK5rWuhArd7H8ffYuY8fg+4mD2i/I",
        vec!["192.42.116.16:443"],
    ),
    fallback(
        "844AE9CAD04325E955E2BE1521563B79FE7094B7",
        "VttOakBYLTZyvVYL4TC9w0lyUGMx0L1gxVsrGUqSato",
        vec!["192.87.28.82:9001", "[2001:678:230:3028:192:87:28:82]:9001"],
    ),
    fallback(
        "855BC2DABE24C861CD887DB9B2E950424B49FC34",
        "eygZLf2qsRh/qNNDSnmm6n6yjbQ14ZCIsPyrz8t4Z3Y",
        vec!["85.228.173.240:443"],
    ),
    fallback(
        "85A885433E50B1874F11CEC9BE98451E24660976",
        "89SDoAQyrQZgvAycXKGw7SO77svcItXDvq35YRbUvjw",
        vec!["178.254.7.88:8443"],
    ),
    fallback(
        "8C00FA7369A7A308F6A137600F0FA07990D9D451",
        "L23HctgabSrEUrcIPyn57lbLCb6h0TOxoYlLovViM4I",
        vec![
            "163.172.194.53:9001",
            "[2001:bc8:225f:142:6c69:7461:7669:73]:9001",
        ],
    ),
    fallback(
        "8CAA470B905758742203E3EB45941719FCA9FEEC",
        "GOC0nmuoMP3c+cpu+iejBmfnw82ySyQJQP+g76QA4cw",
        vec!["188.138.102.98:443"],
    ),
    fallback(
        "8CF987FF43FB7F3D9AA4C4F3D96FFDF247A9A6C2",
        "cfUdtrPAWE4n1l+m0xiXHPiQaX9+b1JYEnsqzSdyA3M",
        vec!["109.70.100.6:443", "[2a03:e600:100::6]:443"],
    ),
    fallback(
        "8D79F73DCD91FC4F5017422FAC70074D6DB8DD81",
        "ZU9W1bA+8qY6At4zhtVyCQF2Y3AC4GA1oo6wKhDyI7Q",
        vec!["5.189.169.190:8080"],
    ),
    fallback(
        "8E6EDA78D8E3ABA88D877C3E37D6D4F0938C7B9F",
        "+cA5po9F08p5F+FsyKqfruH80jejDstEQ0sH7CWvBkM",
        vec!["80.67.172.162:443", "[2001:910:1410:600::1]:443"],
    ),
    fallback(
        "90A5D1355C4B5840E950EB61E673863A6AE3ACA1",
        "OJfFhITpQ0/LsIIcRHh7/w84RNfyAN9zFrUmEb4bNbY",
        vec!["54.37.139.118:9001", "[2001:41d0:601:1100::1b8]:9001"],
    ),
    fallback(
        "924B24AFA7F075D059E8EEB284CC400B33D3D036",
        "7wdm7KIialbA+W4rV3PTbG9w6XHvuHiFTHvzNyQ3LWQ",
        vec!["96.253.78.108:443"],
    ),
    fallback(
        "9661AC95717798884F3E3727D360DD98D66727CC",
        "9qduBqJiROiPxgec0lcTEWj2aNaCyNAz1oHDZ8SnJcw",
        vec!["109.70.100.5:443", "[2a03:e600:100::5]:443"],
    ),
    fallback(
        "99E246DB480B313A3012BC3363093CC26CD209C7",
        "eF/zzvy2OSESbJ7aBF68nk83EOWRt1NunRfkJxY8bOU",
        vec!["173.212.254.192:31337", "[2a02:c207:3002:3972::1]:31337"],
    ),
    fallback(
        "9B2BC7EFD661072AFADC533BE8DCF1C19D8C2DCC",
        "u7iwx1/7QZGzKblwR5uf1k8T+RgHxwaUJb4y07AENVI",
        vec!["188.127.69.60:443", "[2a02:29d0:8008:c0de:bad:beef::]:443"],
    ),
    fallback(
        "9B31F1F1C1554F9FFB3455911F82E818EF7C7883",
        "SA5XFaUqDUFwBwKCdBqpUli5k3mdUuVNUG5jtNQGtd4",
        vec!["185.100.86.128:9001", "[2a06:1700:1::11]:9001"],
    ),
    fallback(
        "9BA84E8C90083676F86C7427C8D105925F13716C",
        "9PB8PEWF0oc4Z74g2B3LoVjNVRhFI4/Ul4e0rbd1UOk",
        vec![
            "95.142.161.63:443",
            "[2001:4b98:dc0:47:216:3eff:fe3d:888c]:443",
        ],
    ),
    fallback(
        "9C900A7F6F5DD034CFFD192DAEC9CCAA813DB022",
        "vFZ3bEHGhGaj44H46UyKeD+fZZ0L4Ny7tliUCYGj37g",
        vec!["86.105.212.130:443"],
    ),
    fallback(
        "9F7D6E6420183C2B76D3CE99624EBC98A21A967E",
        "5E9oJj9elOyWCgvgsUzWpjpG9T5naDb/j9Gq1Wf9aQ0",
        vec!["46.28.110.244:443"],
    ),
    fallback(
        "A0F06C2FADF88D3A39AA3072B406F09D7095AC9E",
        "dJraGedqLgz1nPtZdmW7nFIG/u42ixfcvMKPEly0gwI",
        vec!["46.165.230.5:443"],
    ),
    fallback(
        "A1B28D636A56AAFFE92ADCCA937AA4BD5333BB4C",
        "1HAiAGW/XjYKoJtSnDEOKFe/uRNJxwj3nl/WKd/5ooA",
        vec![
            "193.234.15.55:443",
            "[2a00:1c20:4089:1234:7b2c:11c5:5221:903e]:443",
        ],
    ),
    fallback(
        "A53C46F5B157DD83366D45A8E99A244934A14C46",
        "s0BLh6T2Ruh9+EyOl4tRuv/zVuDxhXRk9S9ILyUjoGc",
        vec!["128.31.0.13:443"],
    ),
    fallback(
        "A68097FE97D3065B1A6F4CE7187D753F8B8513F5",
        "adUTZoPY+JSbZZBen2N/etCSpYTiSk4lb7J7EjrcHUA",
        vec!["212.47.233.86:9101"],
    ),
    fallback(
        "AC2BEDD0BAC72838EA7E6F113F856C4E8018ACDB",
        "O8HMBHhwLmdW9MQvwgHhehwCBB/5VlF92lSPXXT686I",
        vec!["176.10.107.180:9001"],
    ),
    fallback(
        "ACDD9E85A05B127BA010466C13C8C47212E8A38F",
        "KQdIGXATOpoGWI57h8y1uiNoPmHgLLrB/6l7HCZANJ8",
        vec!["185.129.62.62:9001", "[2a06:d380:0:3700::62]:9001"],
    ),
    fallback(
        "AD19490C7DBB26D3A68EFC824F67E69B0A96E601",
        "y8Vd9SygeMOyBkS4oLQVbC0F+9wS4NF5q1P61W80jL0",
        vec![
            "188.40.128.246:9001",
            "[2a01:4f8:221:1ac1:dead:beef:7005:9001]:9001",
        ],
    ),
    fallback(
        "AD86CD1A49573D52A7B6F4A35750F161AAD89C88",
        "2Dlzpo/BG/ltXWCjpT/ovdHkMwkc6GB7USemxQr34R0",
        vec!["176.10.104.240:8443"],
    ),
    fallback(
        "B06F093A3D4DFAD3E923F4F28A74901BD4F74EB1",
        "gLcNnVDNfnWsOPH5MvqkPyn2G+5RkbtBazOuxisZebI",
        vec!["178.17.174.14:9001", "[2a00:1dc0:caff:8b::5b9a]:9001"],
    ),
    fallback(
        "B143D439B72D239A419F8DCE07B8A8EB1B486FA7",
        "K6u+G8wlQS7cMU1QuO4roeUHlj27AbLZMz7a1SriTlk",
        vec!["212.129.62.232:443"],
    ),
    fallback(
        "B27CF1DCEECD50F7992B07D720D7F6BF0EDF9D40",
        "GkEC42Zfp2ZpdkXV/ujA1HhnvpbVbVgOSyl9LDS9es4",
        vec!["109.70.100.2:443", "[2a03:e600:100::2]:443"],
    ),
    fallback(
        "B5212DB685A2A0FCFBAE425738E478D12361710D",
        "T8YqK61rrWPdqYfh2r15KPi/zndb4AdPfATuDvsBQ4I",
        vec!["93.115.97.242:9001"],
    ),
    fallback(
        "B83DC1558F0D34353BB992EF93AFEAFDB226A73E",
        "E22eTrwjkzB05dyqfKAQBgf6VSDa8DRo7kvK0KqFsV0",
        vec!["193.11.114.46:9003"],
    ),
    fallback(
        "B84F248233FEA90CAD439F292556A3139F6E1B82",
        "3uXoHujMGuaUmafqNEKMxdPmYVNZaSMuQSK3y/MOGMc",
        vec!["85.248.227.164:9002", "[2a00:1298:8011:212::164]:9004"],
    ),
    fallback(
        "BCEDF6C193AA687AE471B8A22EBF6BC57C2D285E",
        "zeqbk9w8A3npye2MknX1bn9MvkIRZANMaNi/JOsHmtc",
        vec!["198.96.155.3:5001"],
    ),
    fallback(
        "BCEF908195805E03E92CCFE669C48738E556B9C5",
        "R7L4DDl4sTAMZraCSo+GTcf6b6VatZmVbVbHj4691oM",
        vec!["128.199.55.207:9001", "[2a03:b0c0:2:d0::158:3001]:9001"],
    ),
    fallback(
        "BD552C165E2ED2887D3F1CCE9CFF155DDA2D86E6",
        "ugnOtWVnu78sNpHQU0oOH87SCjQjRlj8EgyW+8+HuP8",
        vec!["213.141.138.174:50001"],
    ),
    fallback(
        "BF0FB582E37F738CD33C3651125F2772705BB8E8",
        "JltQ+aH5D+ZputfajsDp/4G11SPDPSrPComyMc56ALg",
        vec!["148.251.190.229:9010", "[2a01:4f8:211:c68::2]:9010"],
    ),
    fallback(
        "BF735F669481EE1CCC348F0731551C933D1E2278",
        "uxbIFPX2Rnd8yY1ngXHXdDHcShiLNtaEQKLU0XD2kk4",
        vec!["212.47.233.250:9001", "[2001:bc8:4400:2b00::1c:629]:9001"],
    ),
    fallback(
        "C0C4F339046EB824999F711D178472FDF53BE7F5",
        "QlaVAsXYIvGdWJoYwo5fi1MtJ3bQ5plaUjk7NNPVjcQ",
        vec!["132.248.241.5:9101"],
    ),
    fallback(
        "C282248597D1C8522A2A7525E61C8B77BBC37614",
        "wcOYEvHFHT2Nxc+tlzFsklyJtOcc9ckWdBBur+M0SNE",
        vec!["109.70.100.3:443", "[2a03:e600:100::3]:443"],
    ),
    fallback(
        "C36A434DB54C66E1A97A5653858CE36024352C4D",
        "Do0iSNhmPRZU/+vhgczA9Tgg95uKbagg+1JmAreA4BI",
        vec!["50.7.74.170:9001", "[2001:49f0:d002:2::59]:443"],
    ),
    fallback(
        "C414F28FD2BEC1553024299B31D4E726BEB8E788",
        "zr/I8nO0QNvwYJVAJfwEDg1xnQV0xQFsxhovyGidoPk",
        vec!["188.138.112.60:1521"],
    ),
    fallback(
        "C656B41AEFB40A141967EBF49D6E69603C9B4A11",
        "+tt2zAB+flZdk/xAhZMZ//7WUKDmWT6ZQPrmcDsfcN8",
        vec!["178.20.55.18:443"],
    ),
    fallback(
        "C793AB88565DDD3C9E4C6F15CCB9D8C7EF964CE9",
        "Ge0Ts9wi1SVLmpNEmNOP/UQF+ya+AC4ef85oG+V443Y",
        vec!["85.248.227.163:9001", "[2a00:1298:8011:212::163]:9003"],
    ),
    fallback(
        "C87A4D8B534F78FDF0F4639B55F121401FEF259C",
        "i7X2WHCX6qEpIVOIBW0WFfftnTgh4SGLON8y2bJmttU",
        vec!["50.7.74.173:443", "[2001:49f0:d002:2::54]:443"],
    ),
    fallback(
        "CBD0D1BD110EC52963082D839AC6A89D0AE243E7",
        "ZhtxaAA1sDT/zv5RdBo+XfMfX8ue5myekq9IG4YA1P8",
        vec!["176.31.103.150:9001"],
    ),
    fallback(
        "CD0F9AA1A5064430B1DE8E645CBA7A502B27ED5F",
        "+Z4gh7IJ1d3FFPNAdujZsHwl0KWIf0D/MZmApa/6nG4",
        vec![
            "193.234.15.62:443",
            "[2a00:1c20:4089:1234:a6a4:2926:d0af:dfee]:443",
        ],
    ),
    fallback(
        "CE47F0356D86CF0A1A2008D97623216D560FB0A8",
        "c1bbM446BkqMA3IoJsOqoJcCnQhDh7IM5oon2hdSIBU",
        vec!["85.25.213.211:80"],
    ),
    fallback(
        "D1AFBF3117B308B6D1A7AA762B1315FD86A6B8AF",
        "66mkvsATOWGsNMYtnXNadREbESQsPaa39RyFK4os+E4",
        vec!["50.7.74.172:443", "[2001:49f0:d002:2::52]:443"],
    ),
    fallback(
        "D317C7889162E9EC4A1DA1A1095C2A0F377536D9",
        "EUAByQWBvSkMnQNWUlLUh2/QMxtjeRQ1XCvlv8Fkcpk",
        vec!["66.111.2.20:9001"],
    ),
    fallback(
        "D405FCCF06ADEDF898DF2F29C9348DCB623031BA",
        "J5xSAf/GxrnzwL0sQcp36YlGybPxR4iBFQojImbNWKM",
        vec![
            "5.45.111.149:443",
            "[2a03:4000:6:2388:df98:15f9:b34d:443]:443",
        ],
    ),
    fallback(
        "D5C33F3E203728EDF8361EA868B2939CCC43FAFB",
        "vt8/rq91Ra6acbRd4JbcZrg+SVzlsuhI+KepbBN+D2I",
        vec!["12.235.151.200:9029"],
    ),
    fallback(
        "D7082DB97E7F0481CBF4B88CA5F5683399E196A3",
        "/9e91SgkjMz+w20YOF4hpG8Gz71H+BMAGaozyWh7gV4",
        vec!["212.83.166.62:443"],
    ),
    fallback(
        "DB2682153AC0CCAECD2BD1E9EBE99C6815807A1E",
        "C5DCBbKmte43sA0EFfq5REk1UDwEyS9/96lCiF1Bnx0",
        vec!["54.36.237.163:443"],
    ),
    fallback(
        "DD8BD7307017407FCC36F8D04A688F74A0774C02",
        "3YzQBY0tjBYayCeyPJVGtw3sG7D1YSxXI3Gn0QfJeFw",
        vec!["171.25.193.20:443", "[2001:67c:289c::20]:443"],
    ),
    fallback(
        "DDBB2A38252ADDA53E4492DDF982CA6CC6E10EC0",
        "2LuG4YFsqQ63Ym0IP8RfC59zhmuLtC8qN473KOIZkEQ",
        vec![
            "83.212.99.68:443",
            "[2001:648:2ffc:1225:a800:bff:fe3d:67b5]:443",
        ],
    ),
    fallback(
        "E41B16F7DDF52EBB1DB4268AB2FE340B37AD8904",
        "WMgSFE01FsnHfrJocp3do08d7f6hcQKRduv6IR+QUFk",
        vec!["166.70.207.2:9101"],
    ),
    fallback(
        "E51620B90DCB310138ED89EDEDD0A5C361AAE24E",
        "sklrzXlOB7IOPgrFhgK+bfXQiXwVaVCoIjWGOuoXWqg",
        vec!["185.100.86.182:8080"],
    ),
    fallback(
        "E81EF60A73B3809F8964F73766B01BAA0A171E20",
        "ghHxTU+n3zfnrVtsAceD2el+1qAlkpJf8HjAOG1lB/8",
        vec!["212.47.244.38:443"],
    ),
    fallback(
        "E8D114B3C78D8E6E7FEB1004650DD632C2143C9E",
        "e6DDs2vApNKxq0r2vIBmT6rL55Cl3iRtR0f99lgSkqo",
        vec!["185.4.132.148:443", "[2a02:c500:2:f0::5492]:443"],
    ),
    fallback(
        "E947C029087FA1C3499BEF5D4372947C51223D8F",
        "IAOuIm5ABXumGwC+iPWWSqlmLU9YaldnGL6tXcyE10A",
        vec!["195.154.105.170:9001"],
    ),
    fallback(
        "EBE718E1A49EE229071702964F8DB1F318075FF8",
        "CNiUuxHUn0xp1Gfjz+Bzy632Y5rj+W7KtaoEfDfVy58",
        vec![
            "131.188.40.188:11180",
            "[2001:638:a000:4140::ffff:188]:11180",
        ],
    ),
    fallback(
        "ED2338CAC2711B3E331392E1ED2831219B794024",
        "ftz915sTd2+y7JnELlnDnC/jQWdw2iFb4hrykO5V/bM",
        vec!["192.87.28.28:9001", "[2001:678:230:3028:192:87:28:28]:9001"],
    ),
    fallback(
        "EFAE44728264982224445E96214C15F9075DEE1D",
        "s2w6QLaOQFL2tLQoHSS7+hrk42W8RmTK1Q4mp7uasn8",
        vec!["178.20.55.16:443"],
    ),
    fallback(
        "EFEACD781604EB80FBC025EDEDEA2D523AEAAA2F",
        "W3yH/OWbKwWlaP9243SP30qJOf3/0tVvC1M4x8B9u+g",
        vec!["217.182.75.181:9001"],
    ),
    fallback(
        "F24F8BEA2779A79111F33F6832B062BED306B9CB",
        "v6YjNsq1tLsAUt+FlU2uv0Bpkn4iJ9UmkhYAcS3aBJk",
        vec![
            "193.234.15.58:443",
            "[2a00:1c20:4089:1234:cdae:1b3e:cc38:3d45]:443",
        ],
    ),
    fallback(
        "F2DFE5FA1E4CF54F8E761A6D304B9B4EC69BDAE8",
        "ii5cAQdvLGAoaaP0SA2LPBSiqXl8HQNuGR7JeKsi9fU",
        vec![
            "129.13.131.140:443",
            "[2a00:1398:5:f604:cafe:cafe:cafe:9001]:443",
        ],
    ),
    fallback(
        "F4263275CF54A6836EE7BD527B1328836A6F06E1",
        "F2A1yzDIBPuz2W+0RdLorUXzLjfdLIIf9a+cEH8uCuQ",
        vec!["37.187.102.108:443", "[2001:41d0:a:266c::1]:443"],
    ),
    fallback(
        "F4C0EDAA0BF0F7EC138746F8FEF1CE26C7860265",
        "hLfNwGAvN2i5N1ErZM5FqGlYlbxWWqNC5sf5t6ld5KY",
        vec!["5.199.142.236:9001"],
    ),
    fallback(
        "F741E5124CB12700DA946B78C9B2DD175D6CD2A1",
        "iAYLZTzQ8qOBGnbJqBP52JqGWBdUfINS5YiskqOhLLo",
        vec!["163.172.154.162:9001", "[2001:bc8:47a0:162a::1]:9001"],
    ),
    fallback(
        "F8D27B163B9247B232A2EEE68DD8B698695C28DE",
        "ucdzkvv9kNWMbybz4CpMvdGUZPphjv9shfB3DuynaAU",
        vec!["78.47.18.110:80", "[2a01:4f8:120:4023::110]:80"],
    ),
    fallback(
        "F9246DEF2B653807236DA134F2AEAB103D58ABFE",
        "Nk1MWkriq/5vhqfDNBtiJWPTUOGxPi1Xj1ZJBCzSe4c",
        vec!["91.143.88.62:443", "[2a02:180:6:1::3d8]:443"],
    ),
    fallback(
        "FE296180018833AF03A8EACD5894A614623D3F76",
        "h+vpgiit/VapnknS1kv9pCqsJx7XX3WaWpRDKuezmAQ",
        vec!["149.56.45.200:9001", "[2607:5300:201:3000::17d3]:9002"],
    ),
    fallback(
        "FF9FC6D130FA26AE3AE8B23688691DC419F0F22E",
        "47HGw2yL2CRZHbg/22y8GzzQ70fy5sRQL+AfLH8l3XQ",
        vec!["62.141.38.69:443", "[2001:4ba0:cafe:ac5::]:443"],
    ),
    fallback(
        "FFA72BD683BC2FCF988356E6BEC1E490F313FB07",
        "1XEqrNow2olJOwX09N2/ixwregJsPECgqGAOvNMfL5U",
        vec!["193.11.164.243:9001", "[2001:6b0:7:125::243]:9001"],
    ),
]
