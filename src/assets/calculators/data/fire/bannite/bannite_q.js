import { tableNormalA, tableReactionA } from "../../../utils";
import mergeArray from "@util/mergeArray";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

let bannite = charactersData["bannite"];
let qObj = bannite.skill.q;

let skillKeys = [
    {
        key: "dmg1",
        chs: "技能伤害",
        skill: "q",
        element: "fire",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let conste1 = c.constellation >= 1;
    let qLevel = c.skill3 - 1;

    let ret = mergeArray(
        ["chs", skillKeys.map(item => item.chs)],
        ["fire", tableNormalA(attribute, configObject, enemy, skillKeys, "q")],
        ["fireMelt", tableReactionA("melt", attribute, configObject, enemy, skillKeys, "q")],
        ["fireVaporize", tableReactionA("vaporize", attribute, configObject, enemy, skillKeys, "q")],
    );

    let atkBonus = (qObj.ratio[qLevel] + (conste1 ? 0.2 : 0)) * attribute.attackBasic;
    let cure = qObj.cure1[qLevel] * attribute.life() + qObj.cure2[qLevel];

    return {
        damage: ret,
        atkBonus,
        cure,
    }
}