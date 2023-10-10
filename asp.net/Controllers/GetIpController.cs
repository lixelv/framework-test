using Microsoft.AspNetCore.Mvc;
using System.Net;

namespace asp.net.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class GetIpController : ControllerBase
    {
        [HttpGet("get-ip")]
        public IActionResult GetIp()
        {
            var ip = HttpContext.Connection.RemoteIpAddress.ToString();
            return Ok(new { data = new { ip = ip } });
        }
    }
}
