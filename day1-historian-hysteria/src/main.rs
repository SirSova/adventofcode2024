fn main() {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    
    for line in input.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        if parts.len() < 2 {
            continue;
        }
        list1.push(parts[0].parse::<i32>().unwrap());
        list2.push(parts[1].parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();
    
    let mut distance = 0;
    for i in 0..list1.len() {
        let elem1 = list1.get(i).unwrap();
        let elem2 = list2.get(i).unwrap();
        let diff = (elem1 - elem2).abs();
        distance += diff;
    }
    println!("distance: {}", distance);
}

const input: &str = "
99006   28305
38540   91683
18133   49738
70780   13081
38316   55879
61729   73383
57462   59629
91153   56589
76573   52850
76155   36285
24171   76277
10666   30707
73383   93797
86077   24033
40710   50187
58326   65150
67654   30707
64826   52850
70235   11694
66149   71125
24077   19630
46497   93335
78959   47746
49339   29442
54141   95451
96148   29849
60258   88293
23099   57809
41307   66225
65974   82114
40769   84109
65277   79459
20836   39953
12863   96208
30718   30707
65114   50745
46146   36285
14591   81775
99705   90954
64713   86029
18767   78959
50856   44136
19757   47746
89557   74726
54062   80955
69386   30707
91224   49972
68269   75801
47118   94704
49799   57324
42535   91397
69994   50187
52436   94924
24596   56660
86843   95067
96155   75541
25680   75049
10905   30636
73049   91039
35651   82698
13185   61356
47380   49738
11526   78959
22640   17601
28081   24121
91214   38947
70299   56815
69491   64519
14166   11598
44639   11827
39531   65456
29197   97770
97941   39589
89154   67313
74726   62661
42065   57417
67812   40982
17072   26967
73245   20609
11680   79249
68304   94322
98762   25205
51469   55108
93672   47746
19665   81775
16645   69048
58364   31379
92331   50187
97903   65233
86845   79249
50676   93797
75338   48071
87166   34605
89047   94911
33371   43535
90980   44540
92511   54481
47738   98087
81003   30707
87425   53074
86444   36285
24802   51147
26115   76277
89747   63773
46823   81775
70790   54357
45381   40553
71680   21755
56603   12660
20721   25637
54715   73383
70245   32502
95000   47746
61672   49279
69277   80955
18422   20285
89751   24379
82454   81055
18812   71141
12403   50187
51380   52850
70840   30006
56957   80955
80981   75183
42793   45433
40184   68802
75791   61356
54948   15120
25442   67654
88023   11694
34476   29242
73955   43439
40956   31785
59337   60125
84175   16414
61172   79249
20032   96208
11394   97531
89416   73030
58763   39442
71522   96208
41390   29669
89374   91039
45927   12473
13265   75801
19150   40228
82687   11428
44626   49738
44561   79249
97948   51147
14766   15646
38684   16481
74001   52850
75864   40844
65366   71125
53729   67654
90293   52850
37726   97531
70804   85744
10158   95339
46905   21771
71830   22746
26957   30707
62648   69687
16278   15961
72223   84109
68493   27071
63124   14353
39953   61356
46199   93853
51574   11750
56997   83594
15042   93797
29355   85961
32667   81775
91910   96117
45361   90326
57209   42508
21891   82114
69835   36285
94443   91397
55059   40228
15688   87731
32147   88293
86293   69781
34923   63879
44962   23951
82598   96035
40003   76078
13023   81775
27866   11694
67002   88124
91012   77163
66044   48243
33331   56996
83739   40228
46420   37353
94463   32287
89002   11694
11437   89879
22418   47746
21411   15950
32217   84109
88031   28305
35905   13458
35647   65548
51955   40377
74030   74726
81468   75864
43969   88293
46194   82114
85098   86085
19106   76277
39507   81364
49298   88422
46755   75801
65761   78300
77565   69583
99407   22393
71358   75801
40988   49738
15235   49799
85560   82033
22336   50187
30214   76277
33528   47746
65883   73396
28514   43873
54235   49621
40814   84109
30738   61805
23352   88293
74158   76277
74379   75470
13703   24883
65060   51147
98105   12843
63511   82603
14335   55075
95359   82342
88133   61356
95532   43253
51153   75098
14718   62422
13804   98736
91397   49738
43643   47746
76512   75864
64507   68412
56558   50187
94652   62259
80650   42322
56524   75908
97034   70538
78284   49799
63253   17703
70398   91039
20457   87050
38983   60522
80593   79249
32158   98087
15099   13939
78903   12907
62941   80063
11803   11694
49738   37998
27243   61846
93339   79646
68583   81203
46703   84109
66461   76277
50187   98087
15890   71125
52412   65472
88293   89208
44630   55747
85179   97531
52028   81775
84567   52513
19950   72857
51147   65290
34362   91039
83960   35257
17612   86932
44656   11694
33483   36285
51670   49799
96300   75864
95240   36523
21850   67295
52441   81775
32254   81390
68525   52850
74508   53968
80323   67864
65300   47746
24288   44630
66066   51308
16209   83039
29819   30707
74036   70014
62630   84109
34134   28305
46966   41389
73363   76513
95916   40844
21327   76277
37034   55019
80617   75864
24686   77305
25133   61356
93810   79249
68870   14283
38845   52268
41953   30707
76876   75801
20112   68275
91690   79249
12865   26445
35744   94812
49085   75801
86030   26757
88533   76277
43415   47935
34986   87425
99785   61356
91725   49799
30670   61818
94247   22948
96381   48072
59252   26199
17282   80898
45938   10743
79072   40844
55769   75864
78572   88293
87331   83164
60162   75801
71125   75801
40031   18205
52638   38867
44289   84251
63865   49738
18898   49799
95301   88087
22055   67584
53205   56266
28323   52850
90363   14587
89953   75864
73798   75131
45863   98591
98470   18502
71795   68172
33778   62819
98946   74323
21994   95233
30707   48095
51523   42340
71441   84109
33909   93797
10691   73087
29496   95149
42230   40844
68830   80812
79249   28305
44218   37419
79729   96842
32224   75801
30279   78959
38608   56727
45483   76277
32047   40844
70586   44630
76474   40844
32742   78959
98317   11109
42335   53918
65316   70907
81775   96208
96023   23222
12721   73383
73700   12794
79390   75864
90694   98087
91565   82137
86422   53606
92810   28630
74624   59757
68037   37599
28396   33032
89780   68219
30962   99744
23460   67654
77012   93797
21031   52028
66950   55803
65128   75335
28012   11694
57394   80955
98281   46595
77495   26699
74936   37782
91250   12721
91241   77978
91314   40844
88290   91966
20940   29130
94073   32412
27646   61356
60072   84970
24487   40228
93468   98087
34827   84109
42629   54769
72808   15123
67231   70469
47920   51377
95716   77889
14507   84109
38335   71125
59119   85683
27496   30904
49592   36285
59259   92228
14976   12721
32138   60938
49659   75876
11006   51147
27301   81810
59332   11690
91052   97587
63614   74726
13512   96191
85275   75780
19247   28305
40663   71567
92257   20940
68520   34365
87435   87425
59634   73879
48762   66787
37318   77650
49411   74726
34658   71064
35706   84109
93123   61356
17924   52329
87196   66321
57740   49146
22830   53972
51620   18748
75848   28607
14772   65350
18592   91039
66676   44630
44102   82837
47056   61356
86148   93628
40518   50135
41596   40844
78037   80873
35795   71125
18723   62486
93797   67654
97671   69144
23699   43539
97919   93556
16785   82114
62616   10568
15930   25428
82441   50369
32656   84109
77413   49738
72752   93797
25097   68790
19543   50187
75318   43459
19415   84109
20997   33697
35202   23562
72538   32387
44842   47280
28326   62911
17462   40228
42990   61356
20421   52850
17182   78955
49471   43107
31823   84109
90999   49738
52439   75864
47746   51147
50990   20026
26573   61356
38712   56479
69067   80955
36559   61356
81690   17222
52850   47746
82635   51207
76405   46009
78056   97531
77790   16534
97566   28480
91451   40844
30371   97678
31553   92823
40103   96208
96713   84654
72241   20478
21177   52850
36285   49799
60819   83899
89533   96207
52878   70318
98899   79249
21739   95321
52895   32973
24760   44578
61257   84109
59243   84109
83626   49799
73327   76277
89469   30707
38452   72499
91111   70772
19932   29213
23814   67902
11228   98087
90008   84109
71262   91481
75116   76277
21556   28310
67613   87425
18819   32653
16596   47746
60189   48086
91775   74181
82183   26300
82671   22127
27814   70386
56740   51147
23120   49799
57467   97531
46686   73630
75851   28089
43651   40228
97125   95585
49136   66827
91337   50881
94434   49799
32403   76277
41731   98603
27543   78528
74801   94830
14902   39291
99047   84044
66749   79249
76240   44630
72198   92069
95888   76277
32500   11134
86753   75864
15649   40228
56086   37007
15311   76277
37803   96208
53060   91397
51967   46791
17354   75864
10055   91996
34244   51147
21231   29851
59489   85269
13639   52850
34062   76277
83983   88634
93647   71603
53687   27829
70194   63627
98944   51147
13991   69066
87843   28757
74448   18555
52799   77670
75944   81775
42844   93797
96272   49738
17838   28305
26081   59101
45724   61356
47237   93229
21228   49378
35856   48946
93591   79249
61212   48408
61062   49738
96064   67117
75537   91039
44059   71125
81622   40844
50667   78959
97084   72682
13169   50187
57322   40844
12130   79854
82114   50187
54596   37752
47748   49738
24302   70834
39283   75801
44310   49738
82822   28305
79286   11554
78155   29272
26900   79547
97801   17772
62797   75864
80367   71125
52103   31822
43130   57967
28888   68281
73180   98087
82035   44295
25627   28305
10869   98087
37552   41211
84907   80456
34645   49738
61338   18250
83990   52028
11269   79249
79295   40228
43221   28305
86001   84109
34108   96672
95413   40228
19964   95091
82521   79249
60781   52850
57794   49515
12298   52827
84216   75356
36509   47746
82594   91039
32516   34110
50254   39055
35816   98087
60865   16648
61952   46277
53399   47746
77087   63430
96779   11694
13853   62075
22933   98087
58086   11745
21436   49186
50170   49738
86579   76496
26267   11694
77545   75864
43670   81580
21175   71125
87781   88939
52791   71941
48617   97531
96403   49799
67474   86504
98932   75864
46899   15115
79574   20872
30104   28168
76149   47235
70412   85595
33000   91039
59971   27058
88217   85302
15130   21587
56015   12678
54238   84154
35684   52890
88659   73714
40206   75864
81346   69296
35433   44675
84398   98087
64460   63562
32488   47746
77815   35044
61406   76662
93879   88293
33472   39552
86621   96208
31585   68175
92665   60399
51965   89287
99377   98087
20257   42276
79455   93460
44861   63652
94931   44752
34821   16486
34053   28305
35378   45573
99445   47746
30344   62122
68931   70443
57795   16675
51592   57833
52107   56613
44792   99154
81960   56133
93395   49357
53268   60866
30699   47746
44246   50560
71363   76277
84318   85118
50198   93797
58232   93108
76277   21865
22162   48676
77444   25800
77344   77645
84109   84109
64076   88361
56650   19091
69278   39510
78847   99948
81571   76277
44644   18403
90621   79249
59174   91039
43079   96572
46349   32815
69355   75801
67085   50187
66194   84109
91560   87467
96434   67654
57896   60828
12385   56266
13553   51324
39494   81775
33031   72416
84179   75864
76823   79249
78648   30707
49568   67654
13913   52422
13779   30707
19801   51147
53472   76175
54951   79797
78858   71125
33043   42990
66269   76277
69674   52004
56156   38942
15052   93396
74147   98425
16337   77679
57835   19068
35937   65962
79379   88101
37535   42786
48559   88293
72941   42990
13143   61959
98319   42529
21984   99125
52205   59110
37942   51147
30925   80755
85946   17365
29975   80268
16606   67285
10354   87185
43214   15806
75801   71358
85088   87425
23246   66100
14264   51147
97821   80955
90143   66978
43233   85824
19501   40704
62214   18149
70520   24430
98507   11694
57062   20525
19520   81001
45602   98999
22321   95761
58654   51363
40844   25400
99926   80955
27878   52028
92982   71125
24451   96208
24960   93240
78368   88546
73870   49349
64409   90193
68720   35981
84679   18658
85561   69254
61418   25765
23592   52028
79447   53529
79307   47756
43133   52850
67547   71095
25054   99257
18090   49799
42348   55820
53062   75425
12477   51328
32590   30707
43150   85390
90232   46509
54157   70945
10397   23863
40228   92788
10069   44941
82220   79249
51061   49799
24569   52850
92802   52028
21675   45636
55904   91120
32060   61403
99313   63921
64724   32937
41313   25655
92841   52028
48903   61356
32705   49799
29306   30225
12758   69044
92611   94265
37798   11694
72934   71883
17322   91397
31246   51147
42506   74407
91039   47746
41966   74541
37154   37237
80080   21658
25042   57210
63465   40844
51158   61785
71697   40228
68447   67654
59005   11635
72699   49799
77876   47746
32324   47746
80836   30707
79063   19530
92346   15587
98008   66426
52358   23988
61687   71358
19947   96208
57697   78728
81721   72246
46459   57550
25740   54114
78645   76394
80369   31516
33120   69506
86551   79249
79500   44630
87802   36285
96810   95657
63959   52028
36968   33686
56795   50480
12699   18338
80955   87425
54674   68560
89734   52028
56820   40844
33215   40228
51005   59999
54487   93797
40303   60286
89364   90784
99804   67654
47249   98358
13318   11694
71579   47746
42237   82114
10936   52850
94920   41396
97531   27455
55195   28885
37609   76447
32421   95865
86247   11694
75761   97060
92502   36285
24793   76277
79928   55461
61966   30707
54823   96245
12390   75864
98087   51194
28305   50041
11694   30051
25773   14953
10237   72188
53818   84109
90140   57055
16877   76277
70440   91397
20425   65269
31119   96439
64934   16064
41053   43279
59199   52850
96208   55322
47865   28549
72676   99138
36328   83521
87171   87425
33085   48785
29549   11694
16365   71358
21309   32908
78146   38186
77480   87627
33026   82946
78593   18482
51671   32396
24897   14774
66982   40844
10601   56266
90805   78349
98801   75864
69006   79249
87117   90262
51999   52850
69463   56266
79838   75864
87352   21058
70793   51826
65518   42990
63083   33412
14321   51147
71218   91039
68794   49799
61356   72502
29130   30308
50008   34275
75342   36332
22229   22962
43553   91039
56266   15695
26755   40878
43604   79587
69206   14900
83729   11694
97816   40844
74873   56266
96518   96208
26091   53663
35139   75864
77138   84834
63035   67101
90356   11694
64708   90467
68272   91039
87370   11694
81179   11694
91200   91397
68973   85476
44889   52028
56893   37615
28208   66396
16017   49735
33656   40844
95864   81775
85836   40844
94113   39592
38652   40228
59247   46979
64534   71125
83964   40228
15724   56266
55105   29130
13237   28305
34736   99523
95583   12965
34797   36285
47199   88293
20631   56266
";
