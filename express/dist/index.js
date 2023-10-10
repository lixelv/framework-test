"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = __importDefault(require("express"));
const app = (0, express_1.default)();
const PORT = process.env.PORT || 3000;
app.get('/get-ip', (req, res) => {
    const ip = (req.headers['x-forwarded-for'] || req.connection.remoteAddress);
    if (typeof ip === 'string' && ip.startsWith('::ffff:')) {
        const ipv4 = ip.substring(7);
        res.json({ data: { ip: ipv4 } });
    }
    else {
        res.json({ data: { ip } });
    }
});
app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
});
