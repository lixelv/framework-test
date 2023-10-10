import express, { Request, Response } from 'express';

const app = express();

app.get('/get-ip', (req: Request, res: Response) => {
    const ip = req.headers['x-forwarded-for'] || req.connection.remoteAddress;
    res.json({data: {ip: ip}});
});

app.listen(3000, () => {
    console.log('Server is running on port 3000');
});
