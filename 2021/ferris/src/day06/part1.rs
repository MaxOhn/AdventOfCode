#![feature(portable_simd)]

use core_simd::u8x64;

pub fn run(input: &[u8]) -> i64 {
    simd(input)
}

fn unoptimized(input: &[u8]) -> i64 {
    let mut count = [0; 9];

    for i in (0..600).step_by(2) {
        count[(input[i] - b'0') as usize] += 1;
    }

    for day in 0..80 {
        count[(day + 7) % 9] += count[day % 9];
    }

    count.into_iter().sum()
}

fn no_simd(input: &[u8]) -> i64 {
    let mut count = [0; 9];

    unsafe {
        *count.get_unchecked_mut((*input.get_unchecked(0) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(2) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(4) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(6) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(8) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(10) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(12) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(14) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(16) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(18) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(20) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(22) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(24) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(26) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(28) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(30) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(32) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(34) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(36) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(38) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(40) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(42) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(44) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(46) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(48) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(50) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(52) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(54) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(56) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(58) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(60) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(62) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(64) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(66) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(68) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(70) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(72) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(74) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(76) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(78) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(80) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(82) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(84) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(86) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(88) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(90) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(92) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(94) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(96) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(98) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(100) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(102) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(104) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(106) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(108) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(110) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(112) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(114) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(116) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(118) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(120) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(122) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(124) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(126) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(128) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(130) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(132) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(134) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(136) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(138) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(140) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(142) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(144) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(146) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(148) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(150) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(152) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(154) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(156) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(158) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(160) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(162) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(164) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(166) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(168) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(170) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(172) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(174) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(176) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(178) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(180) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(182) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(184) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(186) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(188) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(190) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(192) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(194) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(196) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(198) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(200) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(202) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(204) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(206) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(208) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(210) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(212) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(214) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(216) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(218) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(220) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(222) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(224) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(226) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(228) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(230) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(232) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(234) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(236) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(238) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(240) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(242) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(244) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(246) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(248) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(250) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(252) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(254) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(256) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(258) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(260) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(262) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(264) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(266) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(268) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(270) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(272) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(274) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(276) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(278) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(280) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(282) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(284) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(286) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(288) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(290) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(292) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(294) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(296) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(298) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(300) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(302) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(304) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(306) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(308) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(310) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(312) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(314) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(316) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(318) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(320) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(322) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(324) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(326) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(328) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(330) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(332) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(334) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(336) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(338) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(340) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(342) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(344) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(346) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(348) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(350) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(352) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(354) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(356) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(358) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(360) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(362) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(364) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(366) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(368) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(370) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(372) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(374) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(376) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(378) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(380) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(382) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(384) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(386) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(388) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(390) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(392) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(394) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(396) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(398) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(400) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(402) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(404) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(406) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(408) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(410) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(412) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(414) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(416) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(418) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(420) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(422) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(424) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(426) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(428) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(430) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(432) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(434) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(436) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(438) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(440) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(442) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(444) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(446) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(448) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(450) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(452) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(454) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(456) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(458) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(460) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(462) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(464) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(466) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(468) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(470) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(472) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(474) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(476) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(478) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(480) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(482) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(484) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(486) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(488) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(490) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(492) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(494) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(496) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(498) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(500) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(502) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(504) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(506) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(508) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(510) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(512) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(514) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(516) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(518) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(520) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(522) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(524) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(526) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(528) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(530) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(532) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(534) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(536) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(538) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(540) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(542) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(544) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(546) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(548) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(550) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(552) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(554) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(556) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(558) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(560) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(562) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(564) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(566) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(568) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(570) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(572) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(574) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(576) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(578) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(580) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(582) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(584) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(586) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(588) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(590) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(592) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(594) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(596) - b'0') as usize) += 1;
        *count.get_unchecked_mut((*input.get_unchecked(598) - b'0') as usize) += 1;

        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);

        *count.get_unchecked(0)
            + *count.get_unchecked(1)
            + *count.get_unchecked(2)
            + *count.get_unchecked(3)
            + *count.get_unchecked(4)
            + *count.get_unchecked(5)
            + *count.get_unchecked(6)
            + *count.get_unchecked(7)
            + *count.get_unchecked(8)
    }
}

fn simd(input: &[u8]) -> i64 {
    let mut count = [0; 9];

    unsafe {
        let mut bytes = [
            *input.get_unchecked(0),
            *input.get_unchecked(2),
            *input.get_unchecked(4),
            *input.get_unchecked(6),
            *input.get_unchecked(8),
            *input.get_unchecked(10),
            *input.get_unchecked(12),
            *input.get_unchecked(14),
            *input.get_unchecked(16),
            *input.get_unchecked(18),
            *input.get_unchecked(20),
            *input.get_unchecked(22),
            *input.get_unchecked(24),
            *input.get_unchecked(26),
            *input.get_unchecked(28),
            *input.get_unchecked(30),
            *input.get_unchecked(32),
            *input.get_unchecked(34),
            *input.get_unchecked(36),
            *input.get_unchecked(38),
            *input.get_unchecked(40),
            *input.get_unchecked(42),
            *input.get_unchecked(44),
            *input.get_unchecked(46),
            *input.get_unchecked(48),
            *input.get_unchecked(50),
            *input.get_unchecked(52),
            *input.get_unchecked(54),
            *input.get_unchecked(56),
            *input.get_unchecked(58),
            *input.get_unchecked(60),
            *input.get_unchecked(62),
            *input.get_unchecked(64),
            *input.get_unchecked(66),
            *input.get_unchecked(68),
            *input.get_unchecked(70),
            *input.get_unchecked(72),
            *input.get_unchecked(74),
            *input.get_unchecked(76),
            *input.get_unchecked(78),
            *input.get_unchecked(80),
            *input.get_unchecked(82),
            *input.get_unchecked(84),
            *input.get_unchecked(86),
            *input.get_unchecked(88),
            *input.get_unchecked(90),
            *input.get_unchecked(92),
            *input.get_unchecked(94),
            *input.get_unchecked(96),
            *input.get_unchecked(98),
            *input.get_unchecked(100),
            *input.get_unchecked(102),
            *input.get_unchecked(104),
            *input.get_unchecked(106),
            *input.get_unchecked(108),
            *input.get_unchecked(110),
            *input.get_unchecked(112),
            *input.get_unchecked(114),
            *input.get_unchecked(116),
            *input.get_unchecked(118),
            *input.get_unchecked(120),
            *input.get_unchecked(122),
            *input.get_unchecked(124),
            *input.get_unchecked(126),
        ];

        let arr = (u8x64::from_array(bytes) - u8x64::splat(b'0')).to_array();

        *count.get_unchecked_mut(*arr.get_unchecked(0) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(1) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(2) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(3) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(4) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(5) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(6) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(7) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(8) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(9) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(10) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(11) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(12) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(13) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(14) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(15) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(16) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(17) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(18) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(19) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(20) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(21) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(22) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(23) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(24) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(25) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(26) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(27) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(28) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(29) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(30) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(31) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(32) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(33) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(34) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(35) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(36) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(37) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(38) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(39) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(40) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(41) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(42) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(43) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(44) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(45) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(46) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(47) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(48) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(49) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(50) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(51) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(52) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(53) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(54) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(55) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(56) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(57) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(58) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(59) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(60) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(61) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(62) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(63) as usize) += 1;

        *bytes.get_unchecked_mut(0) = *input.get_unchecked(128);
        *bytes.get_unchecked_mut(1) = *input.get_unchecked(130);
        *bytes.get_unchecked_mut(2) = *input.get_unchecked(132);
        *bytes.get_unchecked_mut(3) = *input.get_unchecked(134);
        *bytes.get_unchecked_mut(4) = *input.get_unchecked(136);
        *bytes.get_unchecked_mut(5) = *input.get_unchecked(138);
        *bytes.get_unchecked_mut(6) = *input.get_unchecked(140);
        *bytes.get_unchecked_mut(7) = *input.get_unchecked(142);
        *bytes.get_unchecked_mut(8) = *input.get_unchecked(144);
        *bytes.get_unchecked_mut(9) = *input.get_unchecked(146);
        *bytes.get_unchecked_mut(10) = *input.get_unchecked(148);
        *bytes.get_unchecked_mut(11) = *input.get_unchecked(150);
        *bytes.get_unchecked_mut(12) = *input.get_unchecked(152);
        *bytes.get_unchecked_mut(13) = *input.get_unchecked(154);
        *bytes.get_unchecked_mut(14) = *input.get_unchecked(156);
        *bytes.get_unchecked_mut(15) = *input.get_unchecked(158);
        *bytes.get_unchecked_mut(16) = *input.get_unchecked(160);
        *bytes.get_unchecked_mut(17) = *input.get_unchecked(162);
        *bytes.get_unchecked_mut(18) = *input.get_unchecked(164);
        *bytes.get_unchecked_mut(19) = *input.get_unchecked(166);
        *bytes.get_unchecked_mut(20) = *input.get_unchecked(168);
        *bytes.get_unchecked_mut(21) = *input.get_unchecked(170);
        *bytes.get_unchecked_mut(22) = *input.get_unchecked(172);
        *bytes.get_unchecked_mut(23) = *input.get_unchecked(174);
        *bytes.get_unchecked_mut(24) = *input.get_unchecked(176);
        *bytes.get_unchecked_mut(25) = *input.get_unchecked(178);
        *bytes.get_unchecked_mut(26) = *input.get_unchecked(180);
        *bytes.get_unchecked_mut(27) = *input.get_unchecked(182);
        *bytes.get_unchecked_mut(28) = *input.get_unchecked(184);
        *bytes.get_unchecked_mut(29) = *input.get_unchecked(186);
        *bytes.get_unchecked_mut(30) = *input.get_unchecked(188);
        *bytes.get_unchecked_mut(31) = *input.get_unchecked(190);
        *bytes.get_unchecked_mut(32) = *input.get_unchecked(192);
        *bytes.get_unchecked_mut(33) = *input.get_unchecked(194);
        *bytes.get_unchecked_mut(34) = *input.get_unchecked(196);
        *bytes.get_unchecked_mut(35) = *input.get_unchecked(198);
        *bytes.get_unchecked_mut(36) = *input.get_unchecked(200);
        *bytes.get_unchecked_mut(37) = *input.get_unchecked(202);
        *bytes.get_unchecked_mut(38) = *input.get_unchecked(204);
        *bytes.get_unchecked_mut(39) = *input.get_unchecked(206);
        *bytes.get_unchecked_mut(40) = *input.get_unchecked(208);
        *bytes.get_unchecked_mut(41) = *input.get_unchecked(210);
        *bytes.get_unchecked_mut(42) = *input.get_unchecked(212);
        *bytes.get_unchecked_mut(43) = *input.get_unchecked(214);
        *bytes.get_unchecked_mut(44) = *input.get_unchecked(216);
        *bytes.get_unchecked_mut(45) = *input.get_unchecked(218);
        *bytes.get_unchecked_mut(46) = *input.get_unchecked(220);
        *bytes.get_unchecked_mut(47) = *input.get_unchecked(222);
        *bytes.get_unchecked_mut(48) = *input.get_unchecked(224);
        *bytes.get_unchecked_mut(49) = *input.get_unchecked(226);
        *bytes.get_unchecked_mut(50) = *input.get_unchecked(228);
        *bytes.get_unchecked_mut(51) = *input.get_unchecked(230);
        *bytes.get_unchecked_mut(52) = *input.get_unchecked(232);
        *bytes.get_unchecked_mut(53) = *input.get_unchecked(234);
        *bytes.get_unchecked_mut(54) = *input.get_unchecked(236);
        *bytes.get_unchecked_mut(55) = *input.get_unchecked(238);
        *bytes.get_unchecked_mut(56) = *input.get_unchecked(240);
        *bytes.get_unchecked_mut(57) = *input.get_unchecked(242);
        *bytes.get_unchecked_mut(58) = *input.get_unchecked(244);
        *bytes.get_unchecked_mut(59) = *input.get_unchecked(246);
        *bytes.get_unchecked_mut(60) = *input.get_unchecked(248);
        *bytes.get_unchecked_mut(61) = *input.get_unchecked(250);
        *bytes.get_unchecked_mut(62) = *input.get_unchecked(252);
        *bytes.get_unchecked_mut(63) = *input.get_unchecked(254);

        let arr = (u8x64::from_array(bytes) - u8x64::splat(b'0')).to_array();

        *count.get_unchecked_mut(*arr.get_unchecked(0) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(1) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(2) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(3) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(4) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(5) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(6) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(7) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(8) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(9) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(10) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(11) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(12) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(13) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(14) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(15) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(16) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(17) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(18) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(19) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(20) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(21) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(22) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(23) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(24) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(25) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(26) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(27) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(28) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(29) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(30) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(31) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(32) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(33) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(34) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(35) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(36) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(37) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(38) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(39) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(40) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(41) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(42) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(43) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(44) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(45) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(46) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(47) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(48) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(49) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(50) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(51) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(52) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(53) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(54) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(55) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(56) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(57) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(58) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(59) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(60) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(61) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(62) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(63) as usize) += 1;

        *bytes.get_unchecked_mut(0) = *input.get_unchecked(256);
        *bytes.get_unchecked_mut(1) = *input.get_unchecked(258);
        *bytes.get_unchecked_mut(2) = *input.get_unchecked(260);
        *bytes.get_unchecked_mut(3) = *input.get_unchecked(262);
        *bytes.get_unchecked_mut(4) = *input.get_unchecked(264);
        *bytes.get_unchecked_mut(5) = *input.get_unchecked(266);
        *bytes.get_unchecked_mut(6) = *input.get_unchecked(268);
        *bytes.get_unchecked_mut(7) = *input.get_unchecked(270);
        *bytes.get_unchecked_mut(8) = *input.get_unchecked(272);
        *bytes.get_unchecked_mut(9) = *input.get_unchecked(274);
        *bytes.get_unchecked_mut(10) = *input.get_unchecked(276);
        *bytes.get_unchecked_mut(11) = *input.get_unchecked(278);
        *bytes.get_unchecked_mut(12) = *input.get_unchecked(280);
        *bytes.get_unchecked_mut(13) = *input.get_unchecked(282);
        *bytes.get_unchecked_mut(14) = *input.get_unchecked(284);
        *bytes.get_unchecked_mut(15) = *input.get_unchecked(286);
        *bytes.get_unchecked_mut(16) = *input.get_unchecked(288);
        *bytes.get_unchecked_mut(17) = *input.get_unchecked(290);
        *bytes.get_unchecked_mut(18) = *input.get_unchecked(292);
        *bytes.get_unchecked_mut(19) = *input.get_unchecked(294);
        *bytes.get_unchecked_mut(20) = *input.get_unchecked(296);
        *bytes.get_unchecked_mut(21) = *input.get_unchecked(298);
        *bytes.get_unchecked_mut(22) = *input.get_unchecked(300);
        *bytes.get_unchecked_mut(23) = *input.get_unchecked(302);
        *bytes.get_unchecked_mut(24) = *input.get_unchecked(304);
        *bytes.get_unchecked_mut(25) = *input.get_unchecked(306);
        *bytes.get_unchecked_mut(26) = *input.get_unchecked(308);
        *bytes.get_unchecked_mut(27) = *input.get_unchecked(310);
        *bytes.get_unchecked_mut(28) = *input.get_unchecked(312);
        *bytes.get_unchecked_mut(29) = *input.get_unchecked(314);
        *bytes.get_unchecked_mut(30) = *input.get_unchecked(316);
        *bytes.get_unchecked_mut(31) = *input.get_unchecked(318);
        *bytes.get_unchecked_mut(32) = *input.get_unchecked(320);
        *bytes.get_unchecked_mut(33) = *input.get_unchecked(322);
        *bytes.get_unchecked_mut(34) = *input.get_unchecked(324);
        *bytes.get_unchecked_mut(35) = *input.get_unchecked(326);
        *bytes.get_unchecked_mut(36) = *input.get_unchecked(328);
        *bytes.get_unchecked_mut(37) = *input.get_unchecked(330);
        *bytes.get_unchecked_mut(38) = *input.get_unchecked(332);
        *bytes.get_unchecked_mut(39) = *input.get_unchecked(334);
        *bytes.get_unchecked_mut(40) = *input.get_unchecked(336);
        *bytes.get_unchecked_mut(41) = *input.get_unchecked(338);
        *bytes.get_unchecked_mut(42) = *input.get_unchecked(340);
        *bytes.get_unchecked_mut(43) = *input.get_unchecked(342);
        *bytes.get_unchecked_mut(44) = *input.get_unchecked(344);
        *bytes.get_unchecked_mut(45) = *input.get_unchecked(346);
        *bytes.get_unchecked_mut(46) = *input.get_unchecked(348);
        *bytes.get_unchecked_mut(47) = *input.get_unchecked(350);
        *bytes.get_unchecked_mut(48) = *input.get_unchecked(352);
        *bytes.get_unchecked_mut(49) = *input.get_unchecked(354);
        *bytes.get_unchecked_mut(50) = *input.get_unchecked(356);
        *bytes.get_unchecked_mut(51) = *input.get_unchecked(358);
        *bytes.get_unchecked_mut(52) = *input.get_unchecked(360);
        *bytes.get_unchecked_mut(53) = *input.get_unchecked(362);
        *bytes.get_unchecked_mut(54) = *input.get_unchecked(364);
        *bytes.get_unchecked_mut(55) = *input.get_unchecked(366);
        *bytes.get_unchecked_mut(56) = *input.get_unchecked(368);
        *bytes.get_unchecked_mut(57) = *input.get_unchecked(370);
        *bytes.get_unchecked_mut(58) = *input.get_unchecked(372);
        *bytes.get_unchecked_mut(59) = *input.get_unchecked(374);
        *bytes.get_unchecked_mut(60) = *input.get_unchecked(376);
        *bytes.get_unchecked_mut(61) = *input.get_unchecked(378);
        *bytes.get_unchecked_mut(62) = *input.get_unchecked(380);
        *bytes.get_unchecked_mut(63) = *input.get_unchecked(382);

        let arr = (u8x64::from_array(bytes) - u8x64::splat(b'0')).to_array();

        *count.get_unchecked_mut(*arr.get_unchecked(0) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(1) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(2) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(3) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(4) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(5) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(6) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(7) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(8) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(9) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(10) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(11) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(12) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(13) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(14) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(15) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(16) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(17) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(18) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(19) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(20) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(21) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(22) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(23) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(24) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(25) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(26) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(27) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(28) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(29) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(30) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(31) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(32) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(33) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(34) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(35) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(36) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(37) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(38) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(39) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(40) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(41) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(42) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(43) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(44) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(45) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(46) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(47) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(48) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(49) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(50) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(51) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(52) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(53) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(54) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(55) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(56) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(57) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(58) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(59) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(60) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(61) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(62) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(63) as usize) += 1;

        // 384..
        *bytes.get_unchecked_mut(0) = *input.get_unchecked(384);
        *bytes.get_unchecked_mut(1) = *input.get_unchecked(386);
        *bytes.get_unchecked_mut(2) = *input.get_unchecked(388);
        *bytes.get_unchecked_mut(3) = *input.get_unchecked(390);
        *bytes.get_unchecked_mut(4) = *input.get_unchecked(392);
        *bytes.get_unchecked_mut(5) = *input.get_unchecked(394);
        *bytes.get_unchecked_mut(6) = *input.get_unchecked(396);
        *bytes.get_unchecked_mut(7) = *input.get_unchecked(398);
        *bytes.get_unchecked_mut(8) = *input.get_unchecked(400);
        *bytes.get_unchecked_mut(9) = *input.get_unchecked(402);
        *bytes.get_unchecked_mut(10) = *input.get_unchecked(404);
        *bytes.get_unchecked_mut(11) = *input.get_unchecked(406);
        *bytes.get_unchecked_mut(12) = *input.get_unchecked(408);
        *bytes.get_unchecked_mut(13) = *input.get_unchecked(410);
        *bytes.get_unchecked_mut(14) = *input.get_unchecked(412);
        *bytes.get_unchecked_mut(15) = *input.get_unchecked(414);
        *bytes.get_unchecked_mut(16) = *input.get_unchecked(416);
        *bytes.get_unchecked_mut(17) = *input.get_unchecked(418);
        *bytes.get_unchecked_mut(18) = *input.get_unchecked(420);
        *bytes.get_unchecked_mut(19) = *input.get_unchecked(422);
        *bytes.get_unchecked_mut(20) = *input.get_unchecked(424);
        *bytes.get_unchecked_mut(21) = *input.get_unchecked(426);
        *bytes.get_unchecked_mut(22) = *input.get_unchecked(428);
        *bytes.get_unchecked_mut(23) = *input.get_unchecked(430);
        *bytes.get_unchecked_mut(24) = *input.get_unchecked(432);
        *bytes.get_unchecked_mut(25) = *input.get_unchecked(434);
        *bytes.get_unchecked_mut(26) = *input.get_unchecked(436);
        *bytes.get_unchecked_mut(27) = *input.get_unchecked(438);
        *bytes.get_unchecked_mut(28) = *input.get_unchecked(440);
        *bytes.get_unchecked_mut(29) = *input.get_unchecked(442);
        *bytes.get_unchecked_mut(30) = *input.get_unchecked(444);
        *bytes.get_unchecked_mut(31) = *input.get_unchecked(446);
        *bytes.get_unchecked_mut(32) = *input.get_unchecked(448);
        *bytes.get_unchecked_mut(33) = *input.get_unchecked(450);
        *bytes.get_unchecked_mut(34) = *input.get_unchecked(452);
        *bytes.get_unchecked_mut(35) = *input.get_unchecked(454);
        *bytes.get_unchecked_mut(36) = *input.get_unchecked(456);
        *bytes.get_unchecked_mut(37) = *input.get_unchecked(458);
        *bytes.get_unchecked_mut(38) = *input.get_unchecked(460);
        *bytes.get_unchecked_mut(39) = *input.get_unchecked(462);
        *bytes.get_unchecked_mut(40) = *input.get_unchecked(464);
        *bytes.get_unchecked_mut(41) = *input.get_unchecked(466);
        *bytes.get_unchecked_mut(42) = *input.get_unchecked(468);
        *bytes.get_unchecked_mut(43) = *input.get_unchecked(470);
        *bytes.get_unchecked_mut(44) = *input.get_unchecked(472);
        *bytes.get_unchecked_mut(45) = *input.get_unchecked(474);
        *bytes.get_unchecked_mut(46) = *input.get_unchecked(476);
        *bytes.get_unchecked_mut(47) = *input.get_unchecked(478);
        *bytes.get_unchecked_mut(48) = *input.get_unchecked(480);
        *bytes.get_unchecked_mut(49) = *input.get_unchecked(482);
        *bytes.get_unchecked_mut(50) = *input.get_unchecked(484);
        *bytes.get_unchecked_mut(51) = *input.get_unchecked(486);
        *bytes.get_unchecked_mut(52) = *input.get_unchecked(488);
        *bytes.get_unchecked_mut(53) = *input.get_unchecked(490);
        *bytes.get_unchecked_mut(54) = *input.get_unchecked(492);
        *bytes.get_unchecked_mut(55) = *input.get_unchecked(494);
        *bytes.get_unchecked_mut(56) = *input.get_unchecked(496);
        *bytes.get_unchecked_mut(57) = *input.get_unchecked(498);
        *bytes.get_unchecked_mut(58) = *input.get_unchecked(500);
        *bytes.get_unchecked_mut(59) = *input.get_unchecked(502);
        *bytes.get_unchecked_mut(60) = *input.get_unchecked(504);
        *bytes.get_unchecked_mut(61) = *input.get_unchecked(506);
        *bytes.get_unchecked_mut(62) = *input.get_unchecked(508);
        *bytes.get_unchecked_mut(63) = *input.get_unchecked(510);

        let arr = (u8x64::from_array(bytes) - u8x64::splat(b'0')).to_array();

        *count.get_unchecked_mut(*arr.get_unchecked(0) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(1) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(2) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(3) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(4) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(5) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(6) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(7) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(8) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(9) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(10) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(11) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(12) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(13) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(14) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(15) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(16) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(17) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(18) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(19) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(20) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(21) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(22) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(23) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(24) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(25) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(26) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(27) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(28) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(29) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(30) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(31) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(32) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(33) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(34) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(35) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(36) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(37) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(38) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(39) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(40) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(41) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(42) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(43) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(44) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(45) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(46) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(47) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(48) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(49) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(50) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(51) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(52) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(53) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(54) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(55) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(56) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(57) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(58) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(59) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(60) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(61) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(62) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(63) as usize) += 1;

        *bytes.get_unchecked_mut(0) = *input.get_unchecked(512);
        *bytes.get_unchecked_mut(1) = *input.get_unchecked(514);
        *bytes.get_unchecked_mut(2) = *input.get_unchecked(516);
        *bytes.get_unchecked_mut(3) = *input.get_unchecked(518);
        *bytes.get_unchecked_mut(4) = *input.get_unchecked(520);
        *bytes.get_unchecked_mut(5) = *input.get_unchecked(522);
        *bytes.get_unchecked_mut(6) = *input.get_unchecked(524);
        *bytes.get_unchecked_mut(7) = *input.get_unchecked(526);
        *bytes.get_unchecked_mut(8) = *input.get_unchecked(528);
        *bytes.get_unchecked_mut(9) = *input.get_unchecked(530);
        *bytes.get_unchecked_mut(10) = *input.get_unchecked(532);
        *bytes.get_unchecked_mut(11) = *input.get_unchecked(534);
        *bytes.get_unchecked_mut(12) = *input.get_unchecked(536);
        *bytes.get_unchecked_mut(13) = *input.get_unchecked(538);
        *bytes.get_unchecked_mut(14) = *input.get_unchecked(540);
        *bytes.get_unchecked_mut(15) = *input.get_unchecked(542);
        *bytes.get_unchecked_mut(16) = *input.get_unchecked(544);
        *bytes.get_unchecked_mut(17) = *input.get_unchecked(546);
        *bytes.get_unchecked_mut(18) = *input.get_unchecked(548);
        *bytes.get_unchecked_mut(19) = *input.get_unchecked(550);
        *bytes.get_unchecked_mut(20) = *input.get_unchecked(552);
        *bytes.get_unchecked_mut(21) = *input.get_unchecked(554);
        *bytes.get_unchecked_mut(22) = *input.get_unchecked(556);
        *bytes.get_unchecked_mut(23) = *input.get_unchecked(558);
        *bytes.get_unchecked_mut(24) = *input.get_unchecked(560);
        *bytes.get_unchecked_mut(25) = *input.get_unchecked(562);
        *bytes.get_unchecked_mut(26) = *input.get_unchecked(564);
        *bytes.get_unchecked_mut(27) = *input.get_unchecked(566);
        *bytes.get_unchecked_mut(28) = *input.get_unchecked(568);
        *bytes.get_unchecked_mut(29) = *input.get_unchecked(570);
        *bytes.get_unchecked_mut(30) = *input.get_unchecked(572);
        *bytes.get_unchecked_mut(31) = *input.get_unchecked(574);
        *bytes.get_unchecked_mut(32) = *input.get_unchecked(576);
        *bytes.get_unchecked_mut(33) = *input.get_unchecked(578);
        *bytes.get_unchecked_mut(34) = *input.get_unchecked(580);
        *bytes.get_unchecked_mut(35) = *input.get_unchecked(582);
        *bytes.get_unchecked_mut(36) = *input.get_unchecked(584);
        *bytes.get_unchecked_mut(37) = *input.get_unchecked(586);
        *bytes.get_unchecked_mut(38) = *input.get_unchecked(588);
        *bytes.get_unchecked_mut(39) = *input.get_unchecked(590);
        *bytes.get_unchecked_mut(40) = *input.get_unchecked(592);
        *bytes.get_unchecked_mut(41) = *input.get_unchecked(594);
        *bytes.get_unchecked_mut(42) = *input.get_unchecked(596);
        *bytes.get_unchecked_mut(43) = *input.get_unchecked(598);

        let arr = (u8x64::from_array(bytes) - u8x64::splat(b'0')).to_array();

        *count.get_unchecked_mut(*arr.get_unchecked(0) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(1) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(2) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(3) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(4) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(5) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(6) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(7) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(8) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(9) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(10) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(11) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(12) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(13) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(14) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(15) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(16) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(17) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(18) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(19) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(20) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(21) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(22) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(23) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(24) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(25) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(26) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(27) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(28) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(29) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(30) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(31) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(32) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(33) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(34) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(35) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(36) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(37) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(38) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(39) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(40) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(41) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(42) as usize) += 1;
        *count.get_unchecked_mut(*arr.get_unchecked(43) as usize) += 1;

        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);
        *count.get_unchecked_mut(6) += *count.get_unchecked(8);
        *count.get_unchecked_mut(7) += *count.get_unchecked(0);
        *count.get_unchecked_mut(8) += *count.get_unchecked(1);
        *count.get_unchecked_mut(0) += *count.get_unchecked(2);
        *count.get_unchecked_mut(1) += *count.get_unchecked(3);
        *count.get_unchecked_mut(2) += *count.get_unchecked(4);
        *count.get_unchecked_mut(3) += *count.get_unchecked(5);
        *count.get_unchecked_mut(4) += *count.get_unchecked(6);
        *count.get_unchecked_mut(5) += *count.get_unchecked(7);

        *count.get_unchecked(0)
            + *count.get_unchecked(1)
            + *count.get_unchecked(2)
            + *count.get_unchecked(3)
            + *count.get_unchecked(4)
            + *count.get_unchecked(5)
            + *count.get_unchecked(6)
            + *count.get_unchecked(7)
            + *count.get_unchecked(8)
    }
}
