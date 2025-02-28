const fs = require('fs/promises');
const path = require('path');
const os = require('os');
const crypto = require('crypto');

async function main() {
    // Get number of files from command line args
    const numFiles = parseInt(process.argv[2]);
    if (isNaN(numFiles)) {
        console.error('Please provide the number of files as an argument');
        process.exit(1);
    }

    // Create temporary files
    const tempFiles = await Promise.all(
        Array(numFiles).fill().map(async (_, i) => {
            const tmpPath = path.join(
                os.tmpdir(),
                `test-${i}-${crypto.randomBytes(6).toString('hex')}.txt`
            );
            await fs.writeFile(tmpPath, '');
            return tmpPath;
        })
    );

    // Measure concurrent file opening
    const startTime = process.hrtime.bigint();
    
    try {
        // Open all files concurrently
        await Promise.all(
            tempFiles.map(filePath => 
                fs.open(filePath, 'r+')
            )
        );

        const endTime = process.hrtime.bigint();
        const elapsedMs = Number(endTime - startTime) / 1_000_000;
        console.log(`Total execution time: ${elapsedMs.toFixed(2)}ms`);
    } finally {
        // Cleanup: remove all temporary files
        await Promise.all(
            tempFiles.map(filePath =>
                fs.unlink(filePath).catch(() => {})
            )
        );
    }
}

main().catch(console.error);
